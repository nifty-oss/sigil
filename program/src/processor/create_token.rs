use pinocchio::{
    instruction::{Seed, Signer},
    pubkey,
};

use super::*;

use crate::{instruction::CreateArgs, state::Token};

pub fn process_create_token(accounts: &[AccountInfo], args: CreateArgs) -> ProgramResult {
    // Accounts.
    let [token_account_info, authority_info, user_info, payer_info, system_program_info] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Account validation.
    assert_signer("payer", payer_info)?;
    assert_empty("token_account", token_account_info)?;
    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;

    let (token_account_pubkey, bump) = pubkey::find_program_address(
        &[
            Pocket::PREFIX,
            authority_info.key().as_ref(),
            user_info.key().as_ref(),
        ],
        &crate::ID,
    );

    // Correct token account.
    assert_same_pubkeys("token_account", token_account_info, &token_account_pubkey)?;

    let pocket_bump = &[bump];
    let signer_seeds = [
        Seed::from(Pocket::PREFIX),
        Seed::from(authority_info.key().as_ref()),
        Seed::from(user_info.key().as_ref()),
        Seed::from(pocket_bump),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    // Create the token account.
    create_account(
        token_account_info,
        payer_info,
        // base len + capacity to hold tokens.
        Pocket::LEN + (std::mem::size_of::<Token>() * args.capacity as usize),
        &crate::ID,
        &signers,
    )?;

    // Get a mutable reference to the account data.
    let account_data = unsafe { token_account_info.borrow_mut_data_unchecked() };

    // Get the mutable byte muck version of the account so we can mutate the data directly.
    let token_authority = PocketMut::from_bytes_mut(account_data);

    // Now can operate on the struct like a normal Rust struct but the bytes are cast directly
    // without deserializ/serializ(ing).
    token_authority.base.set_tag(Tag::Pocket);
    token_authority.base.authority = *authority_info.key();
    token_authority.base.user = *user_info.key();

    Ok(())
}
