use solana_program::{
    program::invoke, rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};
use stevia::collections::u8_avl_tree::U8Node;

use crate::{
    assertions::{assert_non_empty, assert_program_owner},
    error::TokenLiteError,
    instruction::accounts::AddTokenAccounts,
    require, resize_account,
    state::{Mint, TokenAccountMut},
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
    assert_program_owner("mint", mint_info, &crate::ID)?;

    let data = mint_info.data.borrow_mut();
    let mint = Mint::load(&data);

    let account_data = (*token_account_info.data).borrow();
    let token_account = TokenAccount::from_bytes(&account_data);

    // The token account must be associated with the mint via the namespace.
    require!(
        token_account.header.authority == mint.authority,
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
    resize_account!(
        tree_is_full,
        mint.ticker,
        token_account_info,
        payer_info,
        system_program_info
    );

    // Get a mutable reference to the account data.
    let account_data = &mut (*token_account_info.data).borrow_mut();
    let mut token_namespace = TokenAccountMut::from_bytes_mut(account_data);

    // New tokens should start at amount 0.
    token_namespace.tokens.insert(mint.ticker(), 0);

    Ok(())
}
