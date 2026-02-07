use {
    borsh::BorshDeserialize,
    pinocchio::{
        account_info::AccountInfo,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvars::{rent::Rent, Sysvar},
        ProgramResult,
    },
    pinocchio_system::{instructions::Transfer, ID as SYSTEM_PROGRAM_ID},
    stevia::ZeroCopy,
};

use crate::{
    assertions::{
        assert_empty, assert_non_empty, assert_program_owner, assert_same_pubkeys, assert_signer,
    },
    error::SigilError,
    instruction::SigilInstruction,
    require, resize_account,
    state::{Base, Delegation, Mint, MintSeeds, Pocket, PocketMut, Tag, Ticker},
    utils::{close_account, create_account},
};

mod add_token;
mod approve;
mod burn;
mod close_mint;
mod create_mint;
mod create_token;
mod mint_to;
mod revoke;
mod transfer;

use add_token::process_add_token;
use approve::process_approve;
use burn::process_burn;
use close_mint::process_close_mint;
use create_mint::process_create_mint;
use create_token::process_create_token;
use mint_to::process_mint_to;
use revoke::process_revoke;
use transfer::process_transfer;

/// Validates that the signer is either the Pocket owner or a valid delegate for
/// the given ticker. If the signer is the user, returns immediately. If not,
/// checks the delegate_count flag and validates the provided Delegation PDA.
pub fn validate_user_or_delegate(
    signer: &AccountInfo,
    pocket_base: &Base,
    ticker: &Ticker,
    delegation_info: &AccountInfo,
) -> ProgramResult {
    // Fast path: signer is the user.
    if signer.key() == &pocket_base.user {
        return Ok(());
    }

    // Delegate path: count must be > 0.
    if !pocket_base.has_delegates() {
        return Err(SigilError::InvalidDelegate.into());
    }

    // Delegation account must exist and be valid.
    assert_non_empty("delegation", delegation_info)?;
    assert_program_owner("delegation", delegation_info, &crate::ID)?;

    let data = unsafe { delegation_info.borrow_data_unchecked() };
    let delegation = Delegation::load(data);

    if delegation.user != pocket_base.user
        || delegation.authority != pocket_base.authority
        || &delegation.ticker() != ticker
        || delegation.delegate != *signer.key()
    {
        return Err(SigilError::InvalidDelegate.into());
    }

    Ok(())
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: SigilInstruction = SigilInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        SigilInstruction::CreateTokenAccount(args) => {
            msg!("Instruction: Create Token Account");
            process_create_token(accounts, args)
        }
        SigilInstruction::CreateMint(args) => {
            msg!("Instruction: Create Mint");
            process_create_mint(accounts, args)
        }
        SigilInstruction::AddToken => {
            msg!("Instruction: Add Token");
            process_add_token(accounts)
        }
        SigilInstruction::MintTo(args) => {
            msg!("Instruction: Mint To");
            process_mint_to(accounts, args)
        }
        SigilInstruction::Burn(args) => {
            msg!("Instruction: Burn");
            process_burn(accounts, args)
        }
        SigilInstruction::CloseMint => {
            msg!("Instruction: Close Mint");
            process_close_mint(accounts)
        }
        SigilInstruction::Transfer(args) => {
            msg!("Instruction: Transfer");
            process_transfer(accounts, args)
        }
        SigilInstruction::Approve(args) => {
            msg!("Instruction: Approve");
            process_approve(accounts, args)
        }
        SigilInstruction::Revoke(args) => {
            msg!("Instruction: Revoke");
            process_revoke(accounts, args)
        }
    }
}
