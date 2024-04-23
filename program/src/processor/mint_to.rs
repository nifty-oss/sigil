use borsh::BorshSerialize;
use nifty_asset::state::{Asset, Discriminator};
use nifty_asset_interface::{
    extensions::{Blob, BlobBuilder, ExtensionBuilder, Proxy},
    instructions::{UpdateCpi, UpdateInstructionArgs},
    types::{ExtensionInput, ExtensionType},
};
use solana_program::{
    program::invoke, rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};
use stevia::collections::u8_avl_tree::U8Node;

use crate::{
    assertions::{assert_non_empty, assert_program_owner},
    error::TokenLiteError,
    instruction::{accounts::MintToAccounts, MintToArgs},
    require, resize_account,
    state::{MintMetadata, TokenAccountMut, CONTENT_TYPE},
};

use super::*;

pub fn process_mint_to<'a>(accounts: &'a [AccountInfo<'a>], args: MintToArgs) -> ProgramResult {
    // Accounts.
    let ctx = MintToAccounts::context(accounts)?;

    let payer_info = ctx.accounts.payer;
    let namespace_info = ctx.accounts.namespace;
    let mint_info = ctx.accounts.mint;
    let token_account_info = ctx.accounts.token_account;
    let system_program_info = ctx.accounts.system_program;
    let nifty_program_info = ctx.accounts.nifty_program;

    // Account validation.
    if let Some(payer_info) = payer_info {
        assert_signer("payer", payer_info)?;
    }
    assert_signer("namespace", namespace_info)?;

    if let Some(sys_prog_info) = system_program_info {
        assert_same_pubkeys("sys_prog", sys_prog_info, &system_program::ID)?;
    }

    assert_same_pubkeys("nifty_program", nifty_program_info, &nifty_asset::ID)?;

    // The mint account must exist: must have data and be owned by the correct program.
    assert_non_empty("mint", mint_info)?;
    assert_program_owner("mint", mint_info, &nifty_asset::ID)?;

    let asset_data = mint_info.data.borrow_mut();

    // Must be an initialized Nifty asset.
    require!(
        asset_data.len() >= Asset::LEN && asset_data[0] == Discriminator::Asset.into(),
        TokenLiteError::InvalidMint,
        "asset"
    );

    // Must have the proxy extension.
    let proxy = Asset::get::<Proxy>(&asset_data).ok_or(TokenLiteError::InvalidMint)?;

    // The proxy program must match the current program.
    require!(
        proxy.program == &crate::ID,
        TokenLiteError::InvalidMint,
        "proxy program does not match"
    );

    // Must have the blob extension that stores the mint asset_data.
    let blob = Asset::get::<Blob>(&asset_data).ok_or(TokenLiteError::InvalidMint)?;

    let mut metadata =
        MintMetadata::try_from_slice(blob.data).map_err(|_| TokenLiteError::InvalidMint)?;

    let ticker: [u8; 4] = metadata.ticker.as_bytes().try_into().unwrap();
    let namespace = metadata.namespace;

    let account_data = (*token_account_info.data).borrow();
    let token_account = TokenAccount::from_bytes(&account_data);

    // The token account must be associated with the mint via the namespace.
    require!(
        token_account.header.namespace == namespace,
        TokenLiteError::InvalidTokenAccount,
        "token namespace mismatch"
    );

    // Check we can mint the requested amount.
    let new_amount = metadata
        .supply
        .checked_add(args.amount as u64)
        .ok_or(TokenLiteError::NumericalOverflow)?;

    if new_amount > metadata.max_supply {
        return Err(TokenLiteError::MaximumSupplyReached.into());
    }

    let maybe_ticker = token_account.tokens.get(&ticker);
    let tree_is_full = token_account.tokens.is_full();

    drop(account_data);

    match maybe_ticker {
        Some(_) => {
            msg!("Ticker exists, minting tokens to account.");
        }
        None => {
            msg!("Ticker doesn't exist, adding token to account.");

            // Resize if the tree is full.
            resize_account!(
                tree_is_full,
                ticker,
                token_account_info,
                payer_info,
                system_program_info
            );

            // We need a new reference to the recipient account data after the potential resize.
            let mut account_data = (*token_account_info.data).borrow_mut();
            let mut recipient_token_account = TokenAccountMut::from_bytes_mut(&mut account_data);

            // New tokens should start at amount 0.
            recipient_token_account.tokens.insert(ticker, 0);
        }
    }

    // We need a new reference to the recipient account data after the potential resize.
    let mut account_data = (*token_account_info.data).borrow_mut();
    let mut token_account = TokenAccountMut::from_bytes_mut(&mut account_data);

    // Mint the tokens to the token account.
    let amount = token_account.tokens.get_mut(&ticker).unwrap();
    *amount = amount
        .checked_add(args.amount)
        .ok_or(TokenLiteError::NumericalOverflow)?;

    // Update the mint supply.
    metadata.supply = new_amount;

    let data = BlobBuilder::with_capacity(MintMetadata::LEN)
        .set_data(CONTENT_TYPE, &metadata.try_to_vec()?)
        .data();

    let args = UpdateInstructionArgs {
        name: None,
        mutable: None,
        extension: Some(ExtensionInput {
            extension_type: ExtensionType::Blob,
            length: blob.data.len() as u32,
            data: Some(data),
        }),
    };

    // Have to drop this before the CPI.
    drop(asset_data);

    let mut seeds = Vec::with_capacity(32);
    seeds.extend(ticker.iter());
    seeds.extend(namespace_info.key.as_ref()[..28].iter());
    let seeds: &[u8; 32] = seeds.as_slice().try_into().unwrap();

    let (_, bump) = Pubkey::find_program_address(&[seeds], &crate::ID);

    let signer_seeds: &[&[u8]] = &[seeds, &[bump]];

    // Update the Blob with the new mint data.
    UpdateCpi {
        __program: nifty_program_info,
        asset: mint_info,
        authority: namespace_info,
        group: None,
        payer: payer_info,
        system_program: system_program_info,
        __args: args,
        buffer: None,
    }
    .invoke_signed(&[signer_seeds])?;

    Ok(())
}
