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
    state::{Mint, MintSeeds, Pocket, PocketMut, Tag},
    utils::{close_account, create_account},
};

mod add_token;
mod burn;
mod close_mint;
mod create_mint;
mod create_token;
mod mint_to;
mod transfer;

use add_token::process_add_token;
use burn::process_burn;
use close_mint::process_close_mint;
use create_mint::process_create_mint;
use create_token::process_create_token;
use mint_to::process_mint_to;
use transfer::process_transfer;

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
    }
}
