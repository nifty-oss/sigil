use pinocchio::instruction::{Seed, Signer};

use super::*;

use crate::instruction::CreateMintArgs;

pub fn process_create_mint(accounts: &[AccountInfo], args: CreateMintArgs) -> ProgramResult {
    // Accounts.
    let [mint_info, authority_info, payer_info, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Account validation.
    assert_signer("payer", payer_info)?;
    assert_signer("authority", authority_info)?;

    assert_empty("mint_account", mint_info)?;
    assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;

    let ticker: &[u8; 4] = args
        .ticker
        .as_bytes()
        .try_into()
        .map_err(|_| SigilError::InvalidTicker)?;

    let seeds = MintSeeds {
        ticker,
        authority: *authority_info.key(),
    };
    let (mint_pubkey, bump) = Mint::find_pda(&seeds);

    // Correct mint PDA.
    assert_same_pubkeys("mint_account", mint_info, &mint_pubkey)?;

    let mint_bump = &[bump];
    let signer_seeds = [
        Seed::from(Mint::PREFIX),
        Seed::from(seeds.authority.as_ref()),
        Seed::from(seeds.ticker.as_ref()),
        Seed::from(mint_bump),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    // Create the mint account.
    create_account(mint_info, payer_info, Mint::LEN, &crate::ID, &signers)?;

    let mut data = unsafe { mint_info.borrow_mut_data_unchecked() };
    let mint = Mint::load_mut(&mut data);

    // Setter Data
    mint.set_bump(bump);
    mint.set_ticker(*ticker);
    mint.set_tag(Tag::Mint);
    mint.set_decimals(args.decimals);

    // Fields
    mint.authority = *authority_info.key();
    mint.supply = 0;
    mint.max_supply = args.max_supply;

    Ok(())
}
