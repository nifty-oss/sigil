use super::*;

use crate::instruction::{accounts::BurnAccounts, BurnArgs};

pub fn process_burn<'a>(accounts: &'a [AccountInfo<'a>], args: BurnArgs) -> ProgramResult {
    // Accounts.
    let ctx = BurnAccounts::context(accounts)?;

    let user_info = ctx.accounts.user;
    let mint_info = ctx.accounts.mint;
    let token_account_info = ctx.accounts.token_account;

    assert_signer("user", user_info)?;

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &crate::ID)?;

    let mut data = mint_info.data.borrow_mut();
    let mint = Mint::load_mut(&mut data);

    // Token accounts must exist.
    assert_non_empty("token", token_account_info)?;
    assert_program_owner("token", token_account_info, &crate::ID)?;

    let mut account_data = (*token_account_info.data).borrow_mut();
    let mut token_account = PouchMut::from_bytes_mut(&mut account_data);

    // The token accounts must be associated with the mint via the namespace.
    require!(
        token_account.base.authority == mint.authority,
        SigilError::InvalidTokenAccount,
        "token namespace mismatch"
    );
    // The token accounts must be associated with the user and recipient passed in.
    require!(
        &token_account.base.user == user_info.key,
        SigilError::InvalidTokenAccount,
        "token user mismatch"
    );

    // Look up the amount of tokens in the user's account to make sure they have enough to burn.
    let token = match token_account.tokens.get_mut(&mint.ticker().into()) {
        Some(token) => token,
        None => return Err(SigilError::InsufficientFunds.into()),
    };

    // Fail if, trying to burn more than the user has.
    if args.amount > token.amount {
        return Err(SigilError::InsufficientFunds.into());
    }

    // Burn the requested amount.
    token.amount = token
        .amount
        .checked_sub(args.amount)
        .ok_or(SigilError::NumericalOverflow)?;

    // Decrease the mint supply.
    mint.supply = mint
        .supply
        .checked_sub(args.amount as u64)
        .ok_or(SigilError::NumericalOverflow)?;

    Ok(())
}
