use super::*;

use crate::{instruction::TransferArgs, state::Token};

pub fn process_transfer(accounts: &[AccountInfo], args: TransferArgs) -> ProgramResult {
    // Accounts.
    let [user_token_account_info, recipient_token_account_info, user_info, payer_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert_signer("user", user_info)?;

    if payer_info.key() != &crate::ID {
        assert_signer("payer", payer_info)?;
    }
    if system_program_info.key() != &crate::ID {
        assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;
    }

    // Token accounts must exist.
    assert_non_empty("user_token", user_token_account_info)?;
    assert_program_owner("user_token", user_token_account_info, &crate::ID)?;

    assert_non_empty("recipient_token", recipient_token_account_info)?;
    assert_program_owner("recipient_token", recipient_token_account_info, &crate::ID)?;

    let user_account_data = unsafe { user_token_account_info.borrow_mut_data_unchecked() };
    let recipient_account_data =
        unsafe { recipient_token_account_info.borrow_mut_data_unchecked() };

    let mut user_token_account = PocketMut::from_bytes_mut(user_account_data);
    let recipient_token_account = Pocket::from_bytes(recipient_account_data);

    // The token accounts must be in the same namespace.
    if user_token_account.base.authority != recipient_token_account.base.authority {
        return Err(SigilError::InvalidTokenAccount.into());
    }

    // The user passed in must be the actual user on the token account.
    if user_token_account.base.user != *user_info.key() {
        return Err(SigilError::InvalidTokenAccount.into());
    }

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
    let account_data = unsafe { recipient_token_account_info.borrow_mut_data_unchecked() };
    let mut token_account = PocketMut::from_bytes_mut(account_data);

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
