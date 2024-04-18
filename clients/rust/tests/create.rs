#![cfg(feature = "test-sbf")]

use nifty_oss_token_lite::state::{TokenAccount, TokenSeeds};
use nifty_oss_token_lite_client::instructions::CreateBuilder;
use solana_program_test::{tokio, BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

type Result<T> = std::result::Result<T, BanksClientError>;

pub async fn airdrop(
    context: &mut ProgramTestContext,
    receiver: &Pubkey,
    amount: u64,
) -> Result<()> {
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &context.payer.pubkey(),
            receiver,
            amount,
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
    Ok(())
}

#[tokio::test]
async fn create() {
    let mut context = ProgramTest::new(
        "nifty_oss_token_lite",
        nifty_oss_token_lite_client::ID,
        None,
    )
    .start_with_context()
    .await;

    let user_signer = Keypair::new();
    let user = user_signer.pubkey();

    let namespace_signer = context.payer;
    let namespace = namespace_signer.pubkey();

    // Given a PDA derived from the payer's public key.

    let address = TokenAccount::find_pda(TokenSeeds { user, namespace }).0;

    let ix = CreateBuilder::new()
        .token_account(address)
        .user(user)
        .namespace(namespace)
        .payer(namespace)
        .capacity(0)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&namespace),
        &[&namespace_signer, &namespace_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Then an account was created with the correct data.

    let account = context.banks_client.get_account(address).await.unwrap();

    assert!(account.is_some());

    let mut account = account.unwrap();
    assert_eq!(account.data.len(), TokenAccount::LEN);

    let token_account = TokenAccount::from_bytes(&mut account.data);
    assert_eq!(
        token_account.header.key,
        nifty_oss_token_lite::state::Key::TokenAccount
    );
    assert_eq!(token_account.header.namespace, namespace);
}
