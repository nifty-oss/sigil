use nifty_asset::{
    extensions::ProxyBuilder,
    instructions::{CreateCpi, CreateInstructionArgs},
    types::{ExtensionInput, ExtensionType, Standard},
};

use crate::error::TokenLiteError;

use super::*;

pub fn process_create_mint<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: CreateMintArgs,
) -> ProgramResult {
    // Accounts.
    let ctx = CreateMintAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let namespace_info = ctx.accounts.namespace;
    let mint_account_info = ctx.accounts.mint_account;
    let system_program_info = ctx.accounts.system_program;
    let nifty_program_info = ctx.accounts.nifty_program;

    // Account validation.
    assert_signer("payer", payer_info)?;
    assert_signer("namespace", namespace_info)?;

    assert_empty("mint_account", mint_account_info)?;

    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;
    assert_same_pubkeys("nifty_program", nifty_program_info, &nifty_asset::ID)?;

    let ticker_bytes = args.ticker.as_bytes();

    if ticker_bytes.len() != 4 {
        return Err(TokenLiteError::InvalidTicker.into());
    }

    // Seeds should be 32 bytes long, so we take the first 28 bytes of the namespace.
    let mut seeds = Vec::with_capacity(32);
    seeds.extend(ticker_bytes.iter());
    seeds.extend(namespace_info.key.as_ref()[..28].iter());
    let seeds: &[u8; 32] = seeds.as_slice().try_into().unwrap();

    let (mint_account_pubkey, bump) = Pubkey::find_program_address(&[seeds], &crate::ID);

    // Correct mint PDA.
    assert_same_pubkeys("mint_account", mint_account_info, &mint_account_pubkey)?;

    let signer_seeds: &[&[u8]] = &[seeds, &[bump]];

    // Take the ticker bytes and the namespace pubkey and create a string from them: "namespace:ticker".
    let name = format!("{}:{}", namespace_info.key, args.ticker);

    // Proxy extension for the Nifty mint asset.
    let data = ProxyBuilder::with_capacity(100)
        .set(
            &crate::ID,
            seeds,
            bump,
            // "proxy" authority
            Some(ctx.accounts.namespace.key),
        )
        .data();

    let proxy = ExtensionInput {
        extension_type: ExtensionType::Proxy,
        length: data.len() as u32,
        data: Some(data),
    };

    // Create the mint account by CPI'ing into the Nifty program.
    CreateCpi {
        __program: nifty_program_info,
        asset: mint_account_info,
        authority: (namespace_info, true),
        owner: namespace_info,
        group: None,
        group_authority: None,
        payer: Some(payer_info),
        system_program: Some(system_program_info),
        __args: CreateInstructionArgs {
            name,
            standard: Standard::Proxied,
            mutable: true,
            extensions: Some(vec![proxy]),
        },
    }
    .invoke_signed(&[signer_seeds])?;

    Ok(())
}
