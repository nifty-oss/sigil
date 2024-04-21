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
    instruction::accounts::AddTokenAccounts,
    require,
    state::{MintMetadata, TokenAccountMut},
};

use super::*;

pub fn process_add_token<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = AddTokenAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let user_info = ctx.accounts.user;
    let mint_info = ctx.accounts.mint;
    let token_account_info = ctx.accounts.token_account;
    let system_program_info = ctx.accounts.system_program;

    // Correct system program, if passed in.
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

    let account_data = (*token_account_info.data).borrow();
    let token_account = TokenAccount::from_bytes(&account_data);

    // The token account must be associated with the mint via the namespace.
    require!(
        token_account.header.namespace == namespace,
        TokenLiteError::InvalidTokenAccount,
        "token namespace mismatch"
    );

    // The token account must be associated with the user passed in.
    require!(
        &token_account.header.user == user_info.key,
        TokenLiteError::InvalidTokenAccount,
        "token user mismatch"
    );

    let tree_is_full = token_account.tokens.is_full();
    drop(account_data);

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
        let new_len = token_account_info
            .data_len()
            .checked_add(std::mem::size_of::<U8Node<u32, u32>>())
            .ok_or(TokenLiteError::NumericalOverflow)?;

        // Resize the account data.
        token_account_info.realloc(new_len, false)?;

        let rent = Rent::get()?;
        let new_lamports = rent.minimum_balance(new_len);
        let difference = new_lamports
            .checked_sub(token_account_info.lamports())
            .ok_or(TokenLiteError::NumericalOverflow)?;

        invoke(
            &system_instruction::transfer(
                payer_info.key,
                token_account_info.key,
                difference as u64,
            ),
            &[payer_info.clone(), token_account_info.clone()],
        )?;
    }

    // Get a mutable reference to the account data.
    let account_data = &mut (*token_account_info.data).borrow_mut();
    let mut token_namespace = TokenAccountMut::from_bytes_mut(account_data);

    // New tokens should start at amount 0.
    token_namespace.tokens.insert(ticker, 0);

    Ok(())
}
