use solana_program::{
    program::invoke, rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};
use stevia::collections::u8_avl_tree::U8Node;

use crate::{
    assertions::{assert_non_empty, assert_program_owner},
    error::TokenLiteError,
    instruction::{accounts::MintToAccounts, MintToArgs},
    require, resize_account,
    state::{Mint, TokenAccountMut},
};

use super::*;

pub fn process_mint_to<'a>(accounts: &'a [AccountInfo<'a>], args: MintToArgs) -> ProgramResult {
    // Accounts.
    let ctx = MintToAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let authority_info = ctx.accounts.authority;
    let mint_info = ctx.accounts.mint;
    let token_account_info = ctx.accounts.token_account;
    let system_program_info = ctx.accounts.system_program;

    // Account validation.
    if let Some(payer_info) = payer_info {
        assert_signer("payer", payer_info)?;
    }
    assert_signer("authority", authority_info)?;

    if let Some(sys_prog_info) = system_program_info {
        assert_same_pubkeys("sys_prog", sys_prog_info, &system_program::ID)?;
    }

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &crate::ID)?;

    let mut data = (*mint_info.data).borrow_mut();
    let mint = Mint::load_mut(&mut data);
    let ticker = mint.ticker();

    let account_data = (*token_account_info.data).borrow();
    let token_account = TokenAccount::from_bytes(&account_data);

    // The token account must be associated with the mint via the authority.
    require!(
        token_account.header.authority == mint.authority,
        TokenLiteError::InvalidTokenAccount,
        "token authority mismatch"
    );

    // Check we can mint the requested amount.
    let new_amount = mint
        .supply
        .checked_add(args.amount as u64)
        .ok_or(TokenLiteError::NumericalOverflow)?;

    if new_amount > mint.max_supply {
        return Err(TokenLiteError::MaximumSupplyReached.into());
    }

    let maybe_ticker = token_account.tokens.get(&ticker);
    let tree_is_full = token_account.tokens.is_full();

    drop(account_data);

    match maybe_ticker {
        Some(_) => {
            msg!("Ticker exists, minting tokens to account.");
        }
        None => {
            msg!("Ticker doesn't exist, adding token to account.");

            // Resize if the tree is full.
            resize_account!(
                tree_is_full,
                ticker,
                token_account_info,
                payer_info,
                system_program_info
            );

            // We need a new reference to the recipient account data after the potential resize.
            let mut account_data = (*token_account_info.data).borrow_mut();
            let mut recipient_token_account = TokenAccountMut::from_bytes_mut(&mut account_data);

            // New tokens should start at amount 0.
            recipient_token_account.tokens.insert(ticker, 0);
        }
    }

    // We need a new reference to the recipient account data after the potential resize.
    let mut account_data = (*token_account_info.data).borrow_mut();
    let mut token_account = TokenAccountMut::from_bytes_mut(&mut account_data);

    // Mint the tokens to the token account.
    let amount = token_account.tokens.get_mut(&ticker).unwrap();
    *amount = amount
        .checked_add(args.amount)
        .ok_or(TokenLiteError::NumericalOverflow)?;

    // Update the mint supply.
    mint.supply = new_amount;

    Ok(())
}
