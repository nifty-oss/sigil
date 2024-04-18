#![cfg(feature = "test-sbf")]

use nifty_asset::{
    extensions::Proxy,
    state::{Asset, Discriminator, Standard, State},
    ZeroCopy,
};
use nifty_oss_token_lite::state::{TokenAccount, TokenSeeds};
use nifty_oss_token_lite_client::{
    instructions::{CreateMintBuilder, CreateTokenAccountBuilder},
    ID as TokenLiteID,
};
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

async fn program_context() -> ProgramTestContext {
    let mut test = ProgramTest::new(
        "nifty_oss_token_lite",
        nifty_oss_token_lite_client::ID,
        None,
    );
    test.add_program(&nifty_asset::ID.to_string(), nifty_asset::ID, None);
    test.start_with_context().await
}

#[ignore]
#[tokio::test]
async fn create_token_account() {
    let mut context = program_context().await;

    let user_signer = Keypair::new();
    let user = user_signer.pubkey();

    let namespace_signer = context.payer;
    let namespace = namespace_signer.pubkey();

    // Given a PDA derived from the payer's public key.

    let address = TokenAccount::find_pda(TokenSeeds { user, namespace }).0;

    let ix = CreateTokenAccountBuilder::new()
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

#[tokio::test]
async fn create_mint_account() {
    let mut context = program_context().await;

    let namespace_signer = context.payer;
    let namespace = namespace_signer.pubkey();

    // Given a PDA derived from the payer's public key.

    let ticker = String::from("USDC");

    let mut seeds = Vec::with_capacity(32);
    seeds.extend(ticker.as_bytes().iter());
    seeds.extend(namespace.as_ref()[..28].iter());
    let seeds: &[u8; 32] = seeds.as_slice().try_into().unwrap();

    let (address, _) = Pubkey::find_program_address(
        // Seeds should be 32 bytes long, so we take the first 28 bytes of the namespace.
        &[seeds],
        &TokenLiteID,
    );

    let ix = CreateMintBuilder::new()
        .payer(namespace)
        .namespace(namespace)
        .mint_account(address)
        .nifty_program(nifty_asset::ID)
        .ticker(ticker)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&namespace),
        &[&namespace_signer, &namespace_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context.banks_client.get_account(address).await.unwrap();
    assert!(account.is_some());

    let account = account.unwrap();
    let account_data = account.data.as_ref();
    let asset = Asset::load(account_data);

    assert_eq!(asset.discriminator, Discriminator::Asset);
    assert_eq!(asset.state, State::Unlocked);
    assert_eq!(asset.standard, Standard::Proxied);
    assert_eq!(asset.authority, namespace);
    assert_eq!(asset.owner, namespace);

    assert!(Asset::get_extensions(account_data).len() == 1);
    let proxy = Asset::get::<Proxy>(account_data).unwrap();
    assert_eq!(*proxy.authority.value().unwrap(), namespace.into());
    assert_eq!(proxy.program, &TokenLiteID);
}
