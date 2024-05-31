use sigil_client::{accounts::Mint, instructions::CreateMintBuilder};
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
    let test = ProgramTest::new("sigil_program", sigil_client::ID, None);
    test.start_with_context().await
}

pub struct CreateMintParams<'a> {
    pub payer_signer: &'a Keypair,
    pub authority_signer: &'a Keypair,
    pub metadata: &'a TestMetadata,
}

pub struct TestMint {
    pub mint: Pubkey,
}

pub struct TestMetadata {
    pub ticker: String,
    pub max_supply: u64,
    pub decimals: u8,
}

pub async fn create_mint<'a>(
    context: &mut ProgramTestContext,
    params: CreateMintParams<'a>,
) -> Result<TestMint> {
    let CreateMintParams {
        payer_signer,
        authority_signer,
        metadata:
            TestMetadata {
                ticker,
                max_supply,
                decimals,
                ..
            },
    } = params;

    let payer = payer_signer.pubkey();
    let authority = authority_signer.pubkey();

    let mut ticker_seed = [0u8; 4];
    ticker_seed.copy_from_slice(&ticker.as_bytes()[0..4]);

    let (mint, _) = Mint::find_pda(&authority, ticker_seed);

    let ix = CreateMintBuilder::new()
        .payer(payer)
        .authority(authority)
        .mint(mint)
        .ticker(ticker.clone())
        .max_supply(*max_supply)
        .decimals(*decimals)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer),
        &[payer_signer, authority_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    Ok(TestMint { mint })
}
