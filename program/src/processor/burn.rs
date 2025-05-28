use super::*;

use crate::instruction::BurnArgs;

pub fn process_burn(accounts: &[AccountInfo], args: BurnArgs) -> ProgramResult {
    // Accounts.
    // Accounts.
    let [token_account_info, mint_info, user_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert_signer("user", user_info)?;

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &crate::ID)?;

    let data = unsafe { mint_info.borrow_mut_data_unchecked() };
    let mint = Mint::load_mut(data);

    // Token accounts must exist.
    assert_non_empty("token", token_account_info)?;
    assert_program_owner("token", token_account_info, &crate::ID)?;

    let account_data = unsafe { token_account_info.borrow_mut_data_unchecked() };
    let mut token_account = PocketMut::from_bytes_mut(account_data);

    // The token accounts must be associated with the mint via the namespace.
    if token_account.base.authority != mint.authority {
        return Err(SigilError::InvalidTokenAccount.into());
    }

    // The token accounts must be associated with the user and recipient passed in.
    if &token_account.base.user != user_info.key() {
        return Err(SigilError::InvalidTokenAccount.into());
    }

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
