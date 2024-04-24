use super::*;

use crate::{
    error::TokenLiteError,
    state::{Mint, MintSeeds},
};

pub fn process_create_mint<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: CreateMintArgs,
) -> ProgramResult {
    // Accounts.
    let ctx = CreateMintAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let authority_info = ctx.accounts.authority;
    let mint_info = ctx.accounts.mint;
    let system_program_info = ctx.accounts.system_program;

    // Account validation.
    assert_signer("payer", payer_info)?;
    assert_signer("authority", authority_info)?;

    assert_empty("mint_account", mint_info)?;

    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;

    let ticker: &[u8; 4] = args
        .ticker
        .as_bytes()
        .try_into()
        .map_err(|_| TokenLiteError::InvalidTicker)?;

    let seeds = MintSeeds {
        ticker,
        authority: *authority_info.key,
    };
    let (mint_pubkey, bump) = Mint::find_pda(&seeds);

    // Correct mint PDA.
    assert_same_pubkeys("mint_account", mint_info, &mint_pubkey)?;

    let signer_seeds: &[&[u8]] = &[
        Mint::PREFIX,
        seeds.ticker,
        seeds.authority.as_ref(),
        &[bump],
    ];

    msg!("signer seeds length: {}", signer_seeds.len());

    // Create the mint account.
    create_account(
        mint_info,
        payer_info,
        system_program_info,
        Mint::LEN,
        &crate::ID,
        Some(&[signer_seeds]),
    )?;

    msg!("Mint account created.");

    let mut data = (*mint_info.data).borrow_mut();
    let mint = Mint::load_mut(&mut data);

    mint.authority = *authority_info.key;
    mint.ticker = *ticker;
    mint.set_decimals(args.decimals);
    mint.supply = 0;
    mint.max_supply = args.max_supply;

    Ok(())
}
