use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke, pubkey::Pubkey,
    rent::Rent, system_instruction, system_program::ID as SYSTEM_PROGRAM_ID, sysvar::Sysvar,
};
use stevia::{collections::u8_avl_tree::U8Node, ZeroCopy};

use crate::{
    assertions::{
        assert_empty, assert_non_empty, assert_program_owner, assert_same_pubkeys, assert_signer,
    },
    error::SigilError,
    instruction::Instruction,
    require, resize_account,
    state::{Mint, MintSeeds, Tag, TokenAccount, TokenAccountMut},
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
        Instruction::AddToken => {
            msg!("Instruction: Add Token");
            process_add_token(accounts)
        }
        Instruction::MintTo(args) => {
            msg!("Instruction: Mint To");
            process_mint_to(accounts, args)
        }
        Instruction::Burn(args) => {
            msg!("Instruction: Burn");
            process_burn(accounts, args)
        }
        Instruction::CloseMint => {
            msg!("Instruction: Close Mint");
            process_close_mint(accounts)
        }
        Instruction::Transfer(args) => {
            msg!("Instruction: Transfer");
            process_transfer(accounts, args)
        }
    }
}
