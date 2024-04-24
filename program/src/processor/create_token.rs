use crate::state::TokenAccountMut;

use super::*;

pub fn process_create_token<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: CreateArgs,
) -> ProgramResult {
    // Accounts.
    let ctx = CreateTokenAccountAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let user_info = ctx.accounts.user;
    let authority_info = ctx.accounts.authority;
    let token_account_info = ctx.accounts.token_account;
    let system_program_info = ctx.accounts.system_program;

    // Account validation.
    assert_signer("payer", payer_info)?;
    assert_empty("token_account", token_account_info)?;
    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;

    let (token_account_pubkey, bump) = Pubkey::find_program_address(
        &[
            b"token_account",
            user_info.key.as_ref(),
            authority_info.key.as_ref(),
        ],
        &crate::ID,
    );

    // Correct token account.
    assert_same_pubkeys("token_account", token_account_info, &token_account_pubkey)?;

    let signer_seeds: &[&[u8]] = &[
        b"token_account",
        user_info.key.as_ref(),
        authority_info.key.as_ref(),
        &[bump],
    ];

    // Create the token account.
    create_account(
        token_account_info,
        payer_info,
        system_program_info,
        TokenAccount::BASE_LEN,
        &crate::ID,
        Some(&[signer_seeds]),
    )?;

    // Get a mutable reference to the account data.
    let account_data = &mut (*token_account_info.data).borrow_mut();

    // // Get the mutable byte muck version of the account so we can mutate the data directly.
    let mut token_authority = TokenAccountMut::from_bytes_mut(account_data);

    // Now can operate on the struct like a normal Rust struct but the bytes are cast directly
    // without deserializ/serializ(ing).
    token_authority.header.authority = *authority_info.key;
    token_authority.header.user = *user_info.key;
    token_authority.tokens.initialize(args.capacity);

    // No need to serialize the data back into the account, it's already there.

    Ok(())
}
