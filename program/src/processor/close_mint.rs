use super::*;

pub fn process_close_mint(accounts: &[AccountInfo]) -> ProgramResult {
    // Accounts.
    let [mint_info, authority_info, recipient_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Account validation.
    assert_signer("authority", authority_info)?;

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &crate::ID)?;

    let data = unsafe { mint_info.borrow_data_unchecked() };
    let mint = Mint::load(data);

    // The authority must be the authority on the the mint.
    assert_same_pubkeys("authority", authority_info, &mint.authority)?;

    // Mint supply must be zero.
    if mint.supply != 0 {
        return Err(SigilError::MintHasSupply.into());
    }

    let recipient = if recipient_info.key() == &crate::ID {
        authority_info
    } else {
        recipient_info
    };

    close_account(mint_info, recipient)
}
