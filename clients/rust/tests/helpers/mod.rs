use nifty_oss_token_lite_client::{instructions::CreateMintBuilder, ID as TokenLiteID};
use solana_program_test::{BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

type Result<T> = std::result::Result<T, BanksClientError>;

pub trait DirtyClone {
    fn dirty_clone(&self) -> Self;
}

impl DirtyClone for Keypair {
    fn dirty_clone(&self) -> Self {
        let bytes = self.to_bytes();
        Keypair::from_bytes(&bytes).unwrap()
    }
}

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

pub async fn program_context() -> ProgramTestContext {
    let mut test = ProgramTest::new(
        "nifty_oss_token_lite",
        nifty_oss_token_lite_client::ID,
        None,
    );
    test.start_with_context().await
}

pub struct CreateMintParams<'a> {
    pub payer_signer: &'a Keypair,
    pub namespace_signer: &'a Keypair,
    pub ticker: String,
    pub namespace: Pubkey,
    pub max_supply: u64,
    pub decimals: u8,
}

pub struct TestMint {
    pub mint: Pubkey,
    pub metadata: MintMetadata,
}

pub async fn create_mint<'a>(
    context: &mut ProgramTestContext,
    params: CreateMintParams<'a>,
) -> Result<TestMint> {
    let CreateMintParams {
        payer_signer,
        namespace_signer,
        ticker,
        namespace,
        max_supply,
        decimals,
    } = params;

    let payer = payer_signer.pubkey();

    let mut seeds = Vec::with_capacity(32);
    seeds.extend(ticker.as_bytes().iter());
    seeds.extend(namespace.as_ref()[..28].iter());
    let seeds: &[u8; 32] = seeds.as_slice().try_into().unwrap();

    let (mint, _) = Pubkey::find_program_address(
        // Seeds should be 32 bytes long, so we take the first 28 bytes of the namespace.
        &[seeds],
        &TokenLiteID,
    );

    let ix = CreateMintBuilder::new()
        .payer(payer)
        .namespace(namespace)
        .mint(mint)
        .ticker(ticker.clone())
        .max_supply(max_supply)
        .decimals(decimals)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[payer_signer, namespace_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    Ok(TestMint {
        mint,
        metadata: MintMetadata {
            namespace,
            ticker,
            supply: 0,
            max_supply,
            decimals,
        },
    })
}
