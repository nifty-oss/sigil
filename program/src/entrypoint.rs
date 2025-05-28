#![allow(unexpected_cfgs)]

use pinocchio::{account_info::AccountInfo, entrypoint, pubkey::Pubkey, ProgramResult};

use crate::processor;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // TODO: Catch the error so we can print it.
    processor::process_instruction(program_id, accounts, instruction_data)?;
    Ok(())
}
