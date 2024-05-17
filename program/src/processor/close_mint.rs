use super::*;

use crate::instruction::accounts::CloseMintAccounts;

pub fn process_close_mint<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = CloseMintAccounts::context(accounts)?;

    let recipient_info = ctx.accounts.recipient;
    let authority_info = ctx.accounts.authority;
    let mint_info = ctx.accounts.mint;

    // Account validation.
    assert_signer("authority", authority_info)?;

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &crate::ID)?;

    let data = (*mint_info.data).borrow();
    let mint = Mint::load(&data);

    // The authority must be the authority on the the mint.
    assert_same_pubkeys("authority", authority_info, &mint.authority)?;

    // Mint supply must be zero.
    require!(
        mint.supply == 0,
        TokenLiteError::MintHasSupply,
        "mint still has supply"
    );

    drop(data);

    let recipient = recipient_info.unwrap_or(authority_info);

    close_account(mint_info, recipient)
}
