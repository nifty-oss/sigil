use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_ID,
};

use crate::assertions::{assert_empty, assert_same_pubkeys, assert_signer};
use crate::instruction::accounts::CreateAccounts;
use crate::instruction::{CreateArgs, Instruction};
use crate::state::{Key, TokenAccount};
use crate::utils::create_account;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        Instruction::Create(args) => {
            msg!("Instruction: Create");
            create(accounts, args)
        }
    }
}

fn create<'a>(accounts: &'a [AccountInfo<'a>], args: CreateArgs) -> ProgramResult {
    // Accounts.
    let ctx = CreateAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let user_info = ctx.accounts.user;
    let namespace_info = ctx.accounts.namespace;
    let token_account_info = ctx.accounts.token_account;
    let system_program_info = ctx.accounts.system_program;

    // Account validation.
    assert_signer("payer", payer_info)?;
    assert_signer("namespace", namespace_info)?;

    assert_empty("token_account", token_account_info)?;

    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;

    let (token_account_pubkey, bump) = Pubkey::find_program_address(
        &[
            b"token_account",
            user_info.key.as_ref(),
            namespace_info.key.as_ref(),
        ],
        &crate::ID,
    );

    // Correct token account.
    assert_same_pubkeys("token_account", token_account_info, &token_account_pubkey)?;

    let signer_seeds: &[&[u8]] = &[
        b"token_account",
        user_info.key.as_ref(),
        namespace_info.key.as_ref(),
        &[bump],
    ];

    // Create the token account.
    create_account(
        token_account_info,
        payer_info,
        system_program_info,
        TokenAccount::LEN,
        &crate::ID,
        Some(&[signer_seeds]),
    )?;

    // Get a mutable reference to the account data.
    let account_data = &mut (*token_account_info.data).borrow_mut();
    msg!("Account data: {:?}", account_data.len());

    // // Get the mutable byte muck version of the account so we can mutate the data directly.
    let mut token_namespace = TokenAccount::from_bytes(account_data);

    // Now can operate on the struct like a normal Rust struct but the bytes are cast directly
    // without deserializ/serializ(ing).
    msg!("Namespace key: {:?}", namespace_info.key);

    token_namespace.header.key = Key::TokenAccount;
    token_namespace.header.namespace = *namespace_info.key;
    token_namespace.tokens.initialize(args.capacity);

    // No need to serialize the data back into the account, it's already there.

    Ok(())
}
