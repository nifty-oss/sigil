use pinocchio::{
    instruction::{Seed, Signer},
    pubkey,
};

use super::*;

use crate::{instruction::ApproveArgs, state::Delegation};

pub fn process_approve(accounts: &[AccountInfo], args: ApproveArgs) -> ProgramResult {
    // Accounts.
    let [token_account_info, delegation_info, user_info, delegate_info, authority_info, payer_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Account validation.
    assert_signer("user", user_info)?;
    assert_signer("payer", payer_info)?;
    assert_empty("delegation", delegation_info)?;
    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;

    // Token account must exist.
    assert_non_empty("token_account", token_account_info)?;
    assert_program_owner("token_account", token_account_info, &crate::ID)?;

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

    // Derive the Delegation PDA.
    let (delegation_pubkey, bump) = pubkey::find_program_address(
        &[
            Delegation::PREFIX,
            user_info.key().as_ref(),
            authority_info.key().as_ref(),
            &args.ticker,
        ],
        &crate::ID,
    );

    // Validate the delegation account matches the derived PDA.
    assert_same_pubkeys("delegation", delegation_info, &delegation_pubkey)?;

    let delegation_bump = &[bump];
    let signer_seeds = [
        Seed::from(Delegation::PREFIX),
        Seed::from(user_info.key().as_ref()),
        Seed::from(authority_info.key().as_ref()),
        Seed::from(args.ticker.as_ref()),
        Seed::from(delegation_bump),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    // Create the Delegation account.
    create_account(
        delegation_info,
        payer_info,
        Delegation::LEN,
        &crate::ID,
        &signers,
    )?;

    // Initialize the Delegation account.
    let delegation_data = unsafe { delegation_info.borrow_mut_data_unchecked() };
    let delegation = Delegation::load_mut(delegation_data);

    delegation.set_tag(Tag::Delegation);
    delegation.set_bump(bump);
    delegation.set_ticker(args.ticker);
    delegation.delegate = *delegate_info.key();
    delegation.user = *user_info.key();
    delegation.authority = *authority_info.key();

    // Increment the delegate count on the Pocket.
    pocket.base.increment_delegate_count()?;

    Ok(())
}
