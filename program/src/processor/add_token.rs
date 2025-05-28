use super::*;

use crate::state::Token;

pub fn process_add_token<'a>(accounts: &[AccountInfo]) -> ProgramResult {
    // Accounts.
    let [token_account_info, mint_info, user_info, payer_info, system_program_info] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Correct system program, if passed in.
    if system_program_info.key() != &crate::ID {
        assert_same_pubkeys("sys_prog", system_program_info, &SYSTEM_PROGRAM_ID)?;
    }

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &crate::ID)?;

    let data = unsafe { mint_info.borrow_mut_data_unchecked() };
    let mint = Mint::load(&data);

    let account_data = unsafe { token_account_info.borrow_data_unchecked() };
    let token_account = Pocket::from_bytes(&account_data);

    // The token account must be associated with the mint via the namespace.
    if token_account.base.authority != mint.authority {
        return Err(SigilError::InvalidTokenAccount.into());
    }

    // The token account must be associated with the user passed in.
    if &token_account.base.user != user_info.key() {
        return Err(SigilError::InvalidTokenAccount.into());
    }

    let account_is_full = token_account.tokens.is_full();

    // Resize if the tree is full.
    resize_account!(
        account_is_full,
        mint.ticker,
        token_account_info,
        payer_info,
        system_program_info
    );

    // Get a mutable reference to the account data.
    let account_data = unsafe { token_account_info.borrow_mut_data_unchecked() };
    let mut token_namespace = PocketMut::from_bytes_mut(account_data);

    // New tokens should start at amount 0.
    token_namespace.tokens.insert(Token {
        ticker: mint.ticker(),
        amount: 0,
    });

    Ok(())
}
