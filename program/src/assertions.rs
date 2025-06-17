use pinocchio::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

use crate::{error::SigilError, state::Tag};

/// Assert that the given account is owned by the given program.
pub fn assert_program_owner(
    account_name: &str,
    account: &AccountInfo,
    owner: &Pubkey,
) -> ProgramResult {
    if !account.is_owned_by(owner) {
        msg!(&format!(
            "Account \"{}\" [{:?}] expected program owner [{:?}], got [{:?}]",
            account_name,
            account.key(),
            owner,
            unsafe { account.owner() }
        ));
        Err(SigilError::InvalidProgramOwner.into())
    } else {
        Ok(())
    }
}

/// Assert the derivation of the seeds against the given account and return the bump seed.
pub fn assert_pda(
    account_name: &str,
    account: &AccountInfo,
    program_id: &Pubkey,
    seeds: &[&[u8]],
) -> Result<u8, ProgramError> {
    let (key, bump) = pubkey::find_program_address(seeds, program_id);
    if *account.key() != key {
        msg!(&format!(
            "Account \"{}\" [{:?}] is an invalid PDA. Expected the following valid PDA [{:?}]",
            account_name,
            account.key(),
            key,
        ));
        return Err(SigilError::InvalidPda.into());
    }
    Ok(bump)
}

/// Assert that the given account is empty.
pub fn assert_empty(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if !account.data_is_empty() {
        msg!(&format!(
            "Account \"{}\" [{:?}] must be empty",
            account_name,
            account.key(),
        ));
        Err(SigilError::ExpectedEmptyAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account is non empty.
pub fn assert_non_empty(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if account.data_is_empty() {
        msg!(&format!(
            "Account \"{}\" [{:?}] must not be empty",
            account_name,
            account.key(),
        ));
        Err(SigilError::ExpectedNonEmptyAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account is a signer.
pub fn assert_signer(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if !account.is_signer() {
        msg!(&format!(
            "Account \"{}\" [{:?}] must be a signer",
            account_name,
            account.key(),
        ));
        Err(SigilError::ExpectedSignerAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account is writable.
pub fn assert_writable(account_name: &str, account: &AccountInfo) -> ProgramResult {
    if !account.is_writable() {
        msg!(&format!(
            "Account \"{}\" [{:?}] must be writable",
            account_name,
            account.key(),
        ));
        Err(SigilError::ExpectedWritableAccount.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account matches the given public key.
pub fn assert_same_pubkeys(
    account_name: &str,
    account: &AccountInfo,
    expected: &Pubkey,
) -> ProgramResult {
    if account.key() != expected {
        msg!(&format!(
            "Account \"{}\" [{:?}] must match the following public key [{:?}]",
            account_name,
            account.key(),
            expected
        ));
        Err(SigilError::AccountMismatch.into())
    } else {
        Ok(())
    }
}

/// Assert that the given account has the expected account key.
pub fn assert_account_key(account_name: &str, account: &AccountInfo, key: Tag) -> ProgramResult {
    let key_number = key as u8;
    if account.data_len() <= 1 || account.try_borrow_data()?[0] != key_number {
        msg!(&format!(
            "Account \"{}\" [{:?}] expected account key [{:?}], got [{:?}]",
            account_name,
            account.key(),
            key_number,
            account.try_borrow_data()?[0]
        ));
        Err(SigilError::InvalidAccountKey.into())
    } else {
        Ok(())
    }
}

#[macro_export]
macro_rules! require {
    ( $constraint:expr, $error:expr, $message:expr ) => {
        if !$constraint {
            msg!(&format!("Constraint failed: {}", $message));
            return Err($error.into());
        }
    };
    ( $constraint:expr, $error:expr, $message:literal, $($args:tt)+ ) => {
        require!( $constraint, $error, format!($message, $($args)+) );
    };
}
