use {
    pinocchio::{
        account_info::AccountInfo,
        instruction::Signer,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvars::{rent::Rent, Sysvar},
        ProgramResult,
    },
    pinocchio_system::{
        instructions::{CreateAccount, Transfer},
        ID as SYSTEM_PROGRAM_ID,
    },
};

use crate::error::SigilError;

/// Create a new account from the given size.
#[inline(always)]
pub fn create_account(
    target_account: &AccountInfo,
    funding_account: &AccountInfo,
    size: usize,
    owner: &Pubkey,
    signers: &[Signer],
) -> ProgramResult {
    let rent = Rent::get()?;
    let lamports: u64 = rent.minimum_balance(size);

    CreateAccount {
        from: funding_account,
        to: target_account,
        space: size as u64,
        owner,
        lamports,
    }
    .invoke_signed(signers)
}

/// Resize an account using realloc, lifted from Solana Cookbook.
#[inline(always)]
pub fn realloc_account(
    target_account: &AccountInfo,
    funding_account: &AccountInfo,
    new_size: usize,
    refund: bool,
) -> ProgramResult {
    let rent = Rent::get()?;
    let old_minimum_balance = rent.minimum_balance(target_account.data_len());
    let new_minimum_balance = rent.minimum_balance(new_size);
    let lamports_diff = new_minimum_balance.abs_diff(old_minimum_balance);

    if new_minimum_balance > old_minimum_balance {
        Transfer {
            from: funding_account,
            to: target_account,
            lamports: lamports_diff,
        }
        .invoke()?;
    } else if refund {
        transfer_lamports_from_pdas(target_account, funding_account, lamports_diff)?;
    }

    target_account.realloc(new_size, false)
}

/// Close an account.
#[inline(always)]
pub fn close_account(
    // The account to close.
    target_account: &AccountInfo,
    // The account to receive the lamport rent.
    receiving_account: &AccountInfo,
) -> ProgramResult {
    let target_starting_lamports = receiving_account.lamports();
    let mut receiving_account_lamports = receiving_account.try_borrow_mut_lamports()?;
    *receiving_account_lamports = target_starting_lamports
        .checked_add(target_account.lamports())
        .unwrap();

    let mut target_account_lamports = target_account.try_borrow_mut_lamports()?;
    *target_account_lamports = 0;

    unsafe {
        target_account.assign(&SYSTEM_PROGRAM_ID);
    }
    target_account.realloc(0, false)
}

/// Transfer lamports.
#[inline(always)]
pub fn transfer_lamports(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    Transfer { from, to, lamports }.invoke_signed(signers)
}

pub fn transfer_lamports_from_pdas(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
) -> ProgramResult {
    let from_account_starting_lamports = from.lamports();
    let mut from_account_lamports = from.try_borrow_mut_lamports()?;

    *from_account_lamports = from_account_starting_lamports
        .checked_sub(lamports)
        .ok_or::<ProgramError>(SigilError::NumericalOverflow.into())?;

    let to_account_starting_lamports = to.lamports();
    let mut to_account_lamports = to.try_borrow_mut_lamports()?;

    *to_account_lamports = to_account_starting_lamports
        .checked_add(lamports)
        .ok_or::<ProgramError>(SigilError::NumericalOverflow.into())?;

    Ok(())
}

#[macro_export]
macro_rules! resize_account {
    ($tree_is_full:expr, $ticker:expr, $recipient_token_account_info:expr, $payer_info:expr, $system_program_info:expr) => {
        if $tree_is_full {
            // We must reallocate so need a payer and the system program.
            if $payer_info.key() == &crate::ID || $system_program_info.key() == &crate::ID {
                return Err(ProgramError::NotEnoughAccountKeys.into());
            }

            // Get the new length of the account data.
            let new_len = $recipient_token_account_info
                .data_len()
                .checked_add(std::mem::size_of::<Token>())
                .ok_or(SigilError::NumericalOverflow)?;

            // Resize the account data.
            $recipient_token_account_info.realloc(new_len, false)?;

            let rent = Rent::get()?;
            let new_lamports = rent.minimum_balance(new_len);
            let difference = new_lamports
                .checked_sub($recipient_token_account_info.lamports())
                .ok_or(SigilError::NumericalOverflow)?;

            Transfer {
                from: $payer_info,
                to: $recipient_token_account_info,
                lamports: difference as u64,
            }
            .invoke()?;
        }
    };
}
