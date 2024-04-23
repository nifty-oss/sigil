use nifty_asset::{
    extensions::{Blob, Proxy},
    state::{Asset, Discriminator},
};

use crate::{
    assertions::{assert_non_empty, assert_program_owner},
    error::TokenLiteError,
    instruction::{accounts::BurnAccounts, BurnArgs},
    require,
    state::{MintMetadata, TokenAccountMut},
};

use super::*;

pub fn process_burn<'a>(accounts: &'a [AccountInfo<'a>], args: BurnArgs) -> ProgramResult {
    // Accounts.
    let ctx = BurnAccounts::context(accounts)?;

    let user_info = ctx.accounts.user;
    let mint_info = ctx.accounts.mint;
    let token_account_info = ctx.accounts.token_account;

    assert_signer("user", user_info)?;

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &nifty_asset::ID)?;

    let data = mint_info.data.borrow_mut();

    // Must be an initialized Nifty asset.
    require!(
        data.len() >= Asset::LEN && data[0] == Discriminator::Asset.into(),
        TokenLiteError::InvalidMint,
        "asset"
    );

    // Must have the proxy extension.
    let proxy = Asset::get::<Proxy>(&data).ok_or(TokenLiteError::InvalidMint)?;

    // The proxy program must match the current program.
    require!(
        proxy.program == &crate::ID,
        TokenLiteError::InvalidMint,
        "proxy program does not match"
    );

    // Must have the blob extension that stores the mint data.
    let blob = Asset::get::<Blob>(&data).ok_or(TokenLiteError::InvalidMint)?;

    let metadata =
        MintMetadata::try_from_slice(blob.data).map_err(|_| TokenLiteError::InvalidMint)?;

    let ticker: [u8; 4] = metadata.ticker.as_bytes().try_into().unwrap();
    let namespace = metadata.namespace;

    // Token accounts must exist.
    assert_non_empty("token", token_account_info)?;
    assert_program_owner("token", token_account_info, &crate::ID)?;

    let mut account_data = (*token_account_info.data).borrow_mut();
    let mut token_account = TokenAccountMut::from_bytes_mut(&mut account_data);

    // The token accounts must be associated with the mint via the namespace.
    require!(
        token_account.header.namespace == namespace,
        TokenLiteError::InvalidTokenAccount,
        "token namespace mismatch"
    );
    // The token accounts must be associated with the user and recipient passed in.
    require!(
        &token_account.header.user == user_info.key,
        TokenLiteError::InvalidTokenAccount,
        "token user mismatch"
    );

    // Look up the amount of tokens in the user's account to make sure they have enough to burn.
    let amount = match token_account.tokens.get_mut(&ticker) {
        Some(amount) => amount,
        None => return Err(TokenLiteError::InsufficientFunds.into()),
    };

    // Fail if, trying to burn more than the user has.
    if args.amount > *amount {
        return Err(TokenLiteError::InsufficientFunds.into());
    }

    // Burn the requested amount.
    *amount = amount
        .checked_sub(args.amount)
        .ok_or(TokenLiteError::NumericalOverflow)?;

    Ok(())
}
