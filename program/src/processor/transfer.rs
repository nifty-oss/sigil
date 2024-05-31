use super::*;

use crate::{
    instruction::{accounts::TransferAccounts, TransferArgs},
    state::Token,
};

pub fn process_transfer<'a>(accounts: &'a [AccountInfo<'a>], args: TransferArgs) -> ProgramResult {
    // Accounts.
    let ctx = TransferAccounts::context(accounts)?;

    let user_token_account_info = ctx.accounts.user_token_account;
    let recipient_token_account_info = ctx.accounts.recipient_token_account;
    let user_info = ctx.accounts.user;
    let payer_info = ctx.accounts.payer;
    let system_program_info = ctx.accounts.system_program;

    assert_signer("user", user_info)?;

    if let Some(payer_info) = payer_info {
        assert_signer("payer", payer_info)?;
    }
    if let Some(sys_prog_info) = ctx.accounts.system_program {
        assert_same_pubkeys("sys_prog", sys_prog_info, &SYSTEM_PROGRAM_ID)?;
    }

    // Token accounts must exist.
    assert_non_empty("user_token", user_token_account_info)?;
    assert_program_owner("user_token", user_token_account_info, &crate::ID)?;

    assert_non_empty("recipient_token", recipient_token_account_info)?;
    assert_program_owner("recipient_token", recipient_token_account_info, &crate::ID)?;

    let mut user_account_data = (*user_token_account_info.data).borrow_mut();
    let recipient_account_data = (*recipient_token_account_info.data).borrow();

    let mut user_token_account = PocketMut::from_bytes_mut(&mut user_account_data);
    let recipient_token_account = Pocket::from_bytes(&recipient_account_data);

    // The token accounts must be in the same namespace.
    require!(
        user_token_account.base.authority == recipient_token_account.base.authority,
        SigilError::InvalidTokenAccount,
        "token user mismatch"
    );
    // The user passed in must be the actual user on the token account.
    require!(
        user_token_account.base.user == *user_info.key,
        SigilError::InvalidTokenAccount,
        "user authority mismatch"
    );

    // Look up the amount of tokens in the user's account to make sure they have enough to send.
    let source_token = user_token_account
        .tokens
        .get_mut(&args.ticker.into())
        .ok_or(SigilError::InsufficientFunds)?;

    if args.amount > source_token.amount {
        return Err(SigilError::InsufficientFunds.into());
    }
    let tree_is_full = recipient_token_account.tokens.is_full();
    let is_none = recipient_token_account
        .tokens
        .get(&args.ticker.into())
        .is_none();

    // Drop read-only reference to recipient account data.
    drop(recipient_account_data);

    // If the ticker doesn't exist on the recipient's account, add it.
    if is_none && tree_is_full {
        // Resize if the tree is full.
        resize_account!(
            tree_is_full,
            ticker,
            recipient_token_account_info,
            payer_info,
            system_program_info
        );
    }

    // We need a new reference to the recipient account data after the potential resize.
    let mut account_data = (*recipient_token_account_info.data).borrow_mut();
    let mut token_account = PocketMut::from_bytes_mut(&mut account_data);

    if is_none {
        token_account.tokens.insert(Token {
            ticker: args.ticker,
            amount: args.amount,
        });
    } else {
        // We know it exists here so we can unwrap.
        let target_token = token_account.tokens.get_mut(&args.ticker.into()).unwrap();

        target_token.amount = target_token
            .amount
            .checked_add(args.amount)
            .ok_or(SigilError::NumericalOverflow)?;
    }

    // Update the source amount.
    source_token.amount = source_token
        .amount
        .checked_sub(args.amount)
        .ok_or(SigilError::NumericalOverflow)?;

    Ok(())
}
