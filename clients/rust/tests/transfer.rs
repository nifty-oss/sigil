#![cfg(feature = "test-sbf")]

pub mod helpers;
use helpers::program_context;

use sigil_client::{
    accounts::Pocket,
    instructions::{AddTokenBuilder, CreateTokenAccountBuilder, MintToBuilder, TransferBuilder},
};
use solana_program_test::tokio;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};

use crate::helpers::{create_mint, CreateMintParams, DirtyClone, TestMetadata, TestMint};

#[tokio::test]
async fn transfer_tokens() {
    let mut context = program_context().await;

    let payer_signer = context.payer.dirty_clone();
    let payer = payer_signer.pubkey();

    let authority_signer = Keypair::new();
    let authority = authority_signer.pubkey();

    let user_signer = Keypair::new();
    let user = user_signer.pubkey();

    let recipient_signer = Keypair::new();
    let recipient = recipient_signer.pubkey();

    let usdc_metadata = TestMetadata {
        ticker: String::from("USDC"),
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

    // Create user's token account.
    let user_token_account = Pocket::find_pda(&authority, &user).0;

    let ix = CreateTokenAccountBuilder::new()
        .token_account(user_token_account)
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

    // Add USDC token to user's account.
    let ix = AddTokenBuilder::new()
        .payer(Some(payer))
        .user(user)
        .mint(usdc_mint)
        .token_account(user_token_account)
        .system_program(Some(system_program::ID))
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[&payer_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Mint 100 USDC to user.
    let ix = MintToBuilder::new()
        .token_account(user_token_account)
        .mint(usdc_mint)
        .authority(authority)
        .payer(Some(payer))
        .system_program(Some(system_program::ID))
        .amount(100)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[&payer_signer, &authority_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Create recipient's token account.
    let recipient_token_account = Pocket::find_pda(&authority, &recipient).0;

    let ix = CreateTokenAccountBuilder::new()
        .token_account(recipient_token_account)
        .user(recipient)
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

    // Transfer 30 USDC from user to recipient.
    let mut ticker_bytes = [0u8; 4];
    ticker_bytes.copy_from_slice(b"USDC");

    let ix = TransferBuilder::new()
        .user_token_account(user_token_account)
        .recipient_token_account(recipient_token_account)
        .user(user)
        .payer(Some(payer))
        .system_program(Some(system_program::ID))
        .ticker(ticker_bytes)
        .amount(30)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[&payer_signer, &user_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Verify user balance is 70.
    let account = context
        .banks_client
        .get_account(user_token_account)
        .await
        .unwrap()
        .unwrap();
    let pocket = Pocket::from_bytes(&account.data).unwrap();
    assert_eq!(pocket.tokens[0].amount, 70);

    // Verify recipient balance is 30.
    let account = context
        .banks_client
        .get_account(recipient_token_account)
        .await
        .unwrap()
        .unwrap();
    let pocket = Pocket::from_bytes(&account.data).unwrap();
    assert_eq!(pocket.tokens[0].amount, 30);
}
