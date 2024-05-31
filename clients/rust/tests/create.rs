#![cfg(feature = "test-sbf")]

pub mod helpers;
use helpers::program_context;

use sigil_client::{
    accounts::{Mint, Pocket},
    instructions::{AddTokenBuilder, CreateTokenAccountBuilder},
    types::Token,
};
use solana_program_test::tokio;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};

use crate::helpers::{create_mint, CreateMintParams, DirtyClone, TestMetadata, TestMint};

#[tokio::test]
async fn create_mint_account() {
    let mut context = program_context().await;

    let payer_signer = context.payer.dirty_clone();

    let authority_signer = Keypair::new();
    let authority = authority_signer.pubkey();

    let expected_metadata = TestMetadata {
        ticker: String::from("USDC"),
        max_supply: 1_000_000_000,
        decimals: 6,
    };

    let TestMint { mint } = create_mint(
        &mut context,
        CreateMintParams {
            payer_signer: &payer_signer,
            authority_signer: &authority_signer,
            metadata: &expected_metadata,
        },
    )
    .await
    .unwrap();

    let account = context.banks_client.get_account(mint).await.unwrap();
    assert!(account.is_some());

    let account = account.unwrap();
    let account_data = account.data.as_ref();
    let mint = Mint::from_bytes(account_data).unwrap();

    assert_eq!(mint.decimals, expected_metadata.decimals);
    assert_eq!(mint.ticker, expected_metadata.ticker.as_bytes());
    assert_eq!(mint.supply, 0);
    assert_eq!(mint.max_supply, expected_metadata.max_supply);
    assert_eq!(mint.authority, authority);
}

#[tokio::test]
async fn create_token_account() {
    let mut context = program_context().await;

    let payer_signer = context.payer;
    let payer = payer_signer.pubkey();

    let authority_signer = Keypair::new();
    let authority = authority_signer.pubkey();

    let user_signer = Keypair::new();
    let user = user_signer.pubkey();

    // Given a PDA derived from the payer's public key.

    let address = Pocket::find_pda(&authority, &user).0;

    let ix = CreateTokenAccountBuilder::new()
        .token_account(address)
        .user(user)
        .authority(authority)
        .payer(payer)
        .capacity(0)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[&payer_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Then an account was created with the correct data.

    let account = context.banks_client.get_account(address).await.unwrap();

    assert!(account.is_some());

    let account = account.unwrap();
    assert_eq!(account.data.len(), Pocket::LEN);

    let token_account = Pocket::from_bytes(&account.data).unwrap();
    assert_eq!(token_account.authority, authority);
}

#[tokio::test]
async fn add_token() {
    let mut context = program_context().await;

    let payer_signer = context.payer.dirty_clone();
    let payer = payer_signer.pubkey();

    let authority_signer = Keypair::new();
    let authority = authority_signer.pubkey();

    let user_signer = Keypair::new();
    let user = user_signer.pubkey();

    let usdc_metadata = TestMetadata {
        ticker: String::from("USDC"),
        max_supply: 1_000_000_000,
        decimals: 6,
    };

    let bonk_metadata = TestMetadata {
        ticker: String::from("BONK"),
        max_supply: 1_000_000_000,
        decimals: 6,
    };

    let TestMint { mint: usdc_mint } = create_mint(
        &mut context,
        CreateMintParams {
            payer_signer: &payer_signer,
            authority_signer: &authority_signer,
            metadata: &usdc_metadata,
        },
    )
    .await
    .unwrap();

    let TestMint { mint: bonk_mint } = create_mint(
        &mut context,
        CreateMintParams {
            payer_signer: &payer_signer,
            authority_signer: &authority_signer,
            metadata: &bonk_metadata,
        },
    )
    .await
    .unwrap();

    // Find user's token account for the authority.
    let address = Pocket::find_pda(&authority, &user).0;

    // Create the token account.
    let ix = CreateTokenAccountBuilder::new()
        .token_account(address)
        .user(user)
        .authority(authority)
        .payer(payer)
        .capacity(0)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[&payer_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // The account exists...
    let account = context.banks_client.get_account(address).await.unwrap();
    assert!(account.is_some());

    //...and has the base length
    let account = account.unwrap();
    assert_eq!(account.data.len(), Pocket::LEN);

    //...and the expected data.
    let token_account = Pocket::from_bytes(&account.data).unwrap();
    assert_eq!(token_account.authority, authority);

    // Add a token account for USDC mint to the user's token account.

    let ix = AddTokenBuilder::new()
        .payer(Some(payer))
        .user(user)
        .mint(usdc_mint)
        .token_account(address)
        .system_program(Some(system_program::ID))
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[&payer_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context.banks_client.get_account(address).await.unwrap();
    assert!(account.is_some());

    // the account now has additional data the size of one U8Node.
    let account = account.unwrap();
    assert_eq!(
        account.data.len(),
        Pocket::LEN + std::mem::size_of::<Token>()
    );

    // Add a second token to the token account.
    let ix = AddTokenBuilder::new()
        .payer(Some(payer))
        .user(user)
        .mint(bonk_mint)
        .token_account(address)
        .system_program(Some(system_program::ID))
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[&payer_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context.banks_client.get_account(address).await.unwrap();
    assert!(account.is_some());

    // the account now has additional data the size of two U8Node.
    let account = account.unwrap();
    assert_eq!(
        account.data.len(),
        Pocket::LEN + std::mem::size_of::<Token>() * 2
    );
}
