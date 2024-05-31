use super::*;

use crate::{
    instruction::{accounts::MintToAccounts, MintToArgs},
    state::Token,
};

pub fn process_mint_to<'a>(accounts: &'a [AccountInfo<'a>], args: MintToArgs) -> ProgramResult {
    // Accounts.
    let ctx = MintToAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let authority_info = ctx.accounts.authority;
    let mint_info = ctx.accounts.mint;
    let token_account_info = ctx.accounts.token_account;
    let system_program_info = ctx.accounts.system_program;

    // Account validation.
    if let Some(payer_info) = payer_info {
        assert_signer("payer", payer_info)?;
    }
    assert_signer("authority", authority_info)?;

    if let Some(sys_prog_info) = system_program_info {
        assert_same_pubkeys("sys_prog", sys_prog_info, &SYSTEM_PROGRAM_ID)?;
    }

    // The mint and token accounts must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &crate::ID)?;
    assert_non_empty("token", token_account_info)?;
    assert_program_owner("token", token_account_info, &crate::ID)?;

    let mut data = (*mint_info.data).borrow_mut();
    let mint = Mint::load_mut(&mut data);
    let ticker = mint.ticker();

    let account_data = (*token_account_info.data).borrow();
    let token_account = Pocket::from_bytes(&account_data);

    // The token account must be associated with the mint via the authority.
    require!(
        token_account.base.authority == mint.authority,
        SigilError::InvalidTokenAccount,
        "token authority mismatch"
    );

    // Check we can mint the requested amount.
    let new_amount = mint
        .supply
        .checked_add(args.amount as u64)
        .ok_or(SigilError::NumericalOverflow)?;

    if new_amount > mint.max_supply {
        return Err(SigilError::MaximumSupplyReached.into());
    }

    let maybe_ticker = token_account.tokens.get(&ticker.into()).is_some();
    let account_is_full = token_account.tokens.is_full();

    drop(account_data);

    if maybe_ticker {
        msg!("Ticker exists, minting tokens to account.");
    } else {
        msg!("Ticker doesn't exist, adding token to account.");

        // Resize if the tree is full.
        resize_account!(
            account_is_full,
            ticker,
            token_account_info,
            payer_info,
            system_program_info
        );

        // We need a new reference to the recipient account data after the potential resize.
        let mut account_data = (*token_account_info.data).borrow_mut();
        let mut recipient_token_account = PocketMut::from_bytes_mut(&mut account_data);

        // New tokens should start at amount 0.
        recipient_token_account
            .tokens
            .insert(Token { ticker, amount: 0 });
    }

    // We need a new reference to the recipient account data after the potential resize.
    let mut account_data = (*token_account_info.data).borrow_mut();
    let mut token_account = PocketMut::from_bytes_mut(&mut account_data);

    // Mint the tokens to the token account.
    let token = token_account.tokens.get_mut(&ticker.into()).unwrap();
    token.amount = token
        .amount
        .checked_add(args.amount)
        .ok_or(SigilError::NumericalOverflow)?;

    // Update the mint supply.
    mint.supply = new_amount;

    Ok(())
}
