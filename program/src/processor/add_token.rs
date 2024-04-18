use solana_program::{program::invoke, rent::Rent, system_instruction, sysvar::Sysvar};
use stevia::collections::avl_tree::Node;

use crate::{error::TokenLiteError, instruction::AddTokenArgs, state::TokenAccountMut};

use super::*;

pub fn process_add_token<'a>(accounts: &'a [AccountInfo<'a>], args: AddTokenArgs) -> ProgramResult {
    // Accounts.
    let ctx = CreateTokenAccountAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let user_info = ctx.accounts.user;
    let namespace_info = ctx.accounts.namespace;
    let token_account_info = ctx.accounts.token_account;
    let system_program_info = ctx.accounts.system_program;

    // Account validation.
    assert_signer("payer", payer_info)?;
    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;

    let (token_account_pubkey, _) = Pubkey::find_program_address(
        &[
            b"token_account",
            user_info.key.as_ref(),
            namespace_info.key.as_ref(),
        ],
        &crate::ID,
    );

    // Correct token account.
    assert_same_pubkeys("token_account", token_account_info, &token_account_pubkey)?;

    // Validate the ticker length.
    let ticker_bytes = args.ticker.as_bytes();
    if ticker_bytes.len() != 4 {
        return Err(TokenLiteError::InvalidTicker.into());
    }

    let ticker: [u8; 4] = ticker_bytes.try_into().unwrap();

    // Get a mutable reference to the account data.
    let account_data = (*token_account_info.data).borrow();

    // Get the mutable byte muck version of the account so we can mutate the data directly
    let token_namespace = TokenAccount::from_bytes(&account_data);

    let tree_is_full = token_namespace.tokens.is_full();
    drop(account_data);

    // Resize if the tree is full.
    if tree_is_full {
        // Get the new length of the account data.
        let new_len = token_account_info
            .data_len()
            .checked_add(std::mem::size_of::<Node<u32, u32>>())
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
