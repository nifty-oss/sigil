use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_ID,
};

use crate::assertions::{assert_empty, assert_same_pubkeys, assert_signer};
use crate::instruction::accounts::{CreateMintAccounts, CreateTokenAccountAccounts};
use crate::instruction::{CreateArgs, CreateMintArgs, Instruction};
use crate::state::{Key, TokenAccount};
use crate::utils::create_account;

mod create_mint;
mod create_token;

use create_mint::process_create_mint;
use create_token::process_create_token;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        Instruction::CreateTokenAccount(args) => {
            msg!("Instruction: Create Token Account");
            process_create_token(accounts, args)
        }
        Instruction::CreateMint(args) => {
            msg!("Instruction: Create Mint");
            process_create_mint(accounts, args)
        }
    }
}
