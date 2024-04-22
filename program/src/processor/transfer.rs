use nifty_asset::{
    extensions::{Blob, Proxy},
    state::{Asset, Discriminator},
};
use solana_program::{
    program::invoke, rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};
use stevia::collections::u8_avl_tree::U8Node;

use crate::{
    assertions::{assert_non_empty, assert_program_owner},
    error::TokenLiteError,
    instruction::{accounts::TransferAccounts, TransferArgs},
    require,
    state::{MintMetadata, TokenAccountMut},
};

use super::*;

pub fn process_transfer<'a>(accounts: &'a [AccountInfo<'a>], args: TransferArgs) -> ProgramResult {
    // Accounts.
    let ctx = TransferAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let user_info = ctx.accounts.user;
    let recipient_info = ctx.accounts.recipient;
    let mint_info = ctx.accounts.mint;
    let user_token_account_info = ctx.accounts.user_token_account;
    let recipient_token_account_info = ctx.accounts.recipient_token_account;
    let system_program_info = ctx.accounts.system_program;

    if let Some(payer_info) = payer_info {
        assert_signer("payer", payer_info)?;
    }
    if let Some(sys_prog_info) = ctx.accounts.system_program {
        assert_same_pubkeys("sys_prog", sys_prog_info, &system_program::ID)?;
    }

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &nifty_asset::ID)?;

    let data = mint_info.data.borrow_mut();

    // Must be an initialized Nifty asset.
    require!(
        data.len() >= Asset::LEN && data[0] == Discriminator::Asset.into(),
        TokenLiteError::InvalidMint,
        "asset"
    );

    // Must have the proxy extension.
    let proxy = Asset::get::<Proxy>(&data).ok_or(TokenLiteError::InvalidMint)?;

    // The proxy program must match the current program.
    require!(
        proxy.program == &crate::ID,
        TokenLiteError::InvalidMint,
        "proxy program does not match"
    );

    // Must have the blob extension that stores the mint data.
    let blob = Asset::get::<Blob>(&data).ok_or(TokenLiteError::InvalidMint)?;

    let metadata =
        MintMetadata::try_from_slice(blob.data).map_err(|_| TokenLiteError::InvalidMint)?;

    let ticker: [u8; 4] = metadata.ticker.as_bytes().try_into().unwrap();
    let namespace = metadata.namespace;

    // Token accounts must exist.
    assert_non_empty("user_token", user_token_account_info)?;
    assert_program_owner("user_token", user_token_account_info, &crate::ID)?;

    assert_non_empty("recipient_token", recipient_token_account_info)?;
    assert_program_owner("recipient_token", recipient_token_account_info, &crate::ID)?;

    let mut user_account_data = (*user_token_account_info.data).borrow_mut();
    let recipient_account_data = (*recipient_token_account_info.data).borrow();

    let mut user_token_account = TokenAccountMut::from_bytes_mut(&mut user_account_data);
    let recipient_token_account = TokenAccount::from_bytes(&recipient_account_data);

    // The token accounts must be associated with the mint via the namespace.
    require!(
        user_token_account.header.namespace == namespace,
        TokenLiteError::InvalidTokenAccount,
        "token namespace mismatch"
    );
    require!(
        recipient_token_account.header.namespace == namespace,
        TokenLiteError::InvalidTokenAccount,
        "token namespace mismatch"
    );

    // The token accounts must be associated with the user and recipient passed in.
    require!(
        &user_token_account.header.user == user_info.key,
        TokenLiteError::InvalidTokenAccount,
        "token user mismatch"
    );
    require!(
        &recipient_token_account.header.user == recipient_info.key,
        TokenLiteError::InvalidTokenAccount,
        "token user mismatch"
    );

    // Look up the amount of tokens in the user's account to make sure they have enough to send.
    let source_amount = match user_token_account.tokens.get_mut(&ticker) {
        Some(amount) => amount,
        None => return Err(TokenLiteError::InsufficientFunds.into()),
    };

    if args.amount > *source_amount {
        return Err(TokenLiteError::InsufficientFunds.into());
    }

    // If the ticker doesn't exist on the recipient's account, add it.

    match recipient_token_account.tokens.get(&ticker) {
        Some(_) => (),
        None => {
            let tree_is_full = recipient_token_account.tokens.is_full();
            drop(recipient_account_data);

            // Resize if the tree is full.
            if tree_is_full {
                // We must reallocate so need a payer and the system program.
                require!(
                    payer_info.is_some() && system_program_info.is_some(),
                    ProgramError::NotEnoughAccountKeys,
                    "payer and system program required"
                );

                let payer_info = payer_info.unwrap();

                // Get the new length of the account data.
                let new_len = recipient_token_account_info
                    .data_len()
                    .checked_add(std::mem::size_of::<U8Node<u32, u32>>())
                    .ok_or(TokenLiteError::NumericalOverflow)?;

                // Resize the account data.
                recipient_token_account_info.realloc(new_len, false)?;

                let rent = Rent::get()?;
                let new_lamports = rent.minimum_balance(new_len);
                let difference = new_lamports
                    .checked_sub(recipient_token_account_info.lamports())
                    .ok_or(TokenLiteError::NumericalOverflow)?;

                invoke(
                    &system_instruction::transfer(
                        payer_info.key,
                        recipient_token_account_info.key,
                        difference as u64,
                    ),
                    &[payer_info.clone(), recipient_token_account_info.clone()],
                )?;
            }

            // We need a new reference to the recipient account data after the potential resize.
            let mut account_data = (*recipient_token_account_info.data).borrow_mut();
            let mut recipient_token_account = TokenAccountMut::from_bytes_mut(&mut account_data);

            // New tokens should start at amount 0.
            recipient_token_account.tokens.insert(ticker, 0);
        }
    }

    // We need a new reference to the recipient account data after the potential resize.
    let mut account_data = (*recipient_token_account_info.data).borrow_mut();
    let mut token_account = TokenAccountMut::from_bytes_mut(&mut account_data);

    // We know it exists here so we can unwrap.
    let dest_amount = token_account.tokens.get_mut(&ticker).unwrap();

    // Update the token amounts.
    *source_amount = source_amount
        .checked_sub(args.amount)
        .ok_or(TokenLiteError::NumericalOverflow)?;
    *dest_amount = dest_amount
        .checked_add(args.amount)
        .ok_or(TokenLiteError::NumericalOverflow)?;

    Ok(())
}
