use super::*;

use crate::{instruction::ApproveArgs, state::Delegation};

pub fn process_revoke(accounts: &[AccountInfo], args: ApproveArgs) -> ProgramResult {
    // Accounts.
    let [token_account_info, delegation_info, user_info, authority_info, recipient_info] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Account validation.
    assert_signer("user", user_info)?;

    // Token account must exist.
    assert_non_empty("token_account", token_account_info)?;
    assert_program_owner("token_account", token_account_info, &crate::ID)?;

    // Delegation account must exist.
    assert_non_empty("delegation", delegation_info)?;
    assert_program_owner("delegation", delegation_info, &crate::ID)?;

    // Load the Pocket to validate user and authority.
    let account_data = unsafe { token_account_info.borrow_mut_data_unchecked() };
    let pocket = PocketMut::from_bytes_mut(account_data);

    // The user must be the owner of the Pocket.
    require!(
        pocket.base.user == *user_info.key(),
        SigilError::InvalidTokenAccount,
        "user mismatch"
    );

    // The authority must match the Pocket's authority.
    require!(
        pocket.base.authority == *authority_info.key(),
        SigilError::InvalidTokenAccount,
        "authority mismatch"
    );

    // Validate the Delegation account matches user, authority, and ticker.
    {
        let delegation_data = unsafe { delegation_info.borrow_data_unchecked() };
        let delegation = Delegation::load(delegation_data);

        require!(
            delegation.tag() == Tag::Delegation,
            SigilError::InvalidDelegate,
            "not a delegation account"
        );

        require!(
            delegation.user == *user_info.key(),
            SigilError::InvalidDelegate,
            "delegation user mismatch"
        );

        require!(
            delegation.authority == *authority_info.key(),
            SigilError::InvalidDelegate,
            "delegation authority mismatch"
        );

        require!(
            delegation.ticker() == args.ticker,
            SigilError::InvalidDelegate,
            "delegation ticker mismatch"
        );
    }

    // Determine the recipient for the refunded rent.
    let recipient = if recipient_info.key() == &crate::ID {
        user_info
    } else {
        recipient_info
    };

    // Close the Delegation account, refunding rent.
    close_account(delegation_info, recipient)?;

    // Decrement the delegate count on the Pocket.
    pocket.base.decrement_delegate_count()?;

    Ok(())
}
