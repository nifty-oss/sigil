// #![cfg(feature = "test-sbf")]

// use borsh::BorshDeserialize;
// use nifty_asset::{
//     extensions::{Blob, Proxy},
//     state::{Asset, Tag, Standard, State},
//     ZeroCopy,
// };
// use sigil_program::state::{MintMetadata, TokenAccount, TokenSeeds};
// use sigil_client::{
//     instructions::{AddTokenBuilder, CreateTokenAccountBuilder},
//     ID as SigilID,
// };
// use solana_program_test::tokio;
// use solana_sdk::{
//     signature::{Keypair, Signer},
//     system_program,
//     transaction::Transaction,
// };
// use stevia::collections::u8_avl_tree::U8Node;

// pub mod helpers;

// use helpers::program_context;

// use crate::helpers::{create_mint, CreateMintParams, DirtyClone, TestMint};

// #[tokio::test]
// async fn transfer_tokens() {
//     let mut context = program_context().await;

//     let payer_signer = context.payer.dirty_clone();
//     let payer = payer_signer.pubkey();

//     let namespace_signer = Keypair::new();
//     let namespace = namespace_signer.pubkey();

//     let user_signer = Keypair::new();
//     let user = user_signer.pubkey();

//     let TestMint {
//         mint: usdc_mint,
//         metadata: _,
//     } = create_mint(
//         &mut context,
//         CreateMintParams {
//             payer_signer: &payer_signer,
//             namespace_signer: &namespace_signer,
//             ticker: String::from("USDC"),
//             namespace,
//             max_supply: 1_000_000_000,
//             decimals: 6,
//         },
//     )
//     .await
//     .unwrap();

//     let TestMint {
//         mint: bonk_mint,
//         metadata: _,
//     } = create_mint(
//         &mut context,
//         CreateMintParams {
//             payer_signer: &payer_signer,
//             namespace_signer: &namespace_signer,
//             ticker: String::from("BONK"),
//             namespace,
//             max_supply: 1_000_000_000,
//             decimals: 6,
//         },
//     )
//     .await
//     .unwrap();

//     // Find user's token account for the namespace.
//     let address = TokenAccount::find_pda(TokenSeeds { user, namespace }).0;

//     // Create the token account.
//     let ix = CreateTokenAccountBuilder::new()
//         .token_account(address)
//         .user(user)
//         .namespace(namespace)
//         .payer(payer)
//         .capacity(0)
//         .instruction();

//     let tx = Transaction::new_signed_with_payer(
//         &[ix],
//         Some(&payer),
//         &[&payer_signer],
//         context.last_blockhash,
//     );
//     context.banks_client.process_transaction(tx).await.unwrap();

//     // The account exists...
//     let account = context.banks_client.get_account(address).await.unwrap();
//     assert!(account.is_some());

//     //...and has the base length
//     let account = account.unwrap();
//     assert_eq!(account.data.len(), TokenAccount::BASE_LEN);

//     //...and the expected data.
//     let token_account = TokenAccount::from_bytes(&account.data);
//     assert_eq!(token_account.header.namespace, namespace);

//     // Add a token account for USDC mint to the user's token account.

//     let ix = AddTokenBuilder::new()
//         .payer(Some(payer))
//         .user(user)
//         .mint(usdc_mint)
//         .token_account(address)
//         .system_program(Some(system_program::ID))
//         .instruction();

//     let tx = Transaction::new_signed_with_payer(
//         &[ix],
//         Some(&payer),
//         &[&payer_signer],
//         context.last_blockhash,
//     );
//     context.banks_client.process_transaction(tx).await.unwrap();

//     let account = context.banks_client.get_account(address).await.unwrap();
//     assert!(account.is_some());

//     // the account now has additional data the size of one U8Node.
//     let account = account.unwrap();
//     assert_eq!(
//         account.data.len(),
//         TokenAccount::BASE_LEN + std::mem::size_of::<U8Node<u32, u32>>()
//     );

//     // Add a second token to the token account.
//     let ix = AddTokenBuilder::new()
//         .payer(Some(payer))
//         .user(user)
//         .mint(bonk_mint)
//         .token_account(address)
//         .system_program(Some(system_program::ID))
//         .instruction();

//     let tx = Transaction::new_signed_with_payer(
//         &[ix],
//         Some(&payer),
//         &[&payer_signer],
//         context.last_blockhash,
//     );
//     context.banks_client.process_transaction(tx).await.unwrap();

//     let account = context.banks_client.get_account(address).await.unwrap();
//     assert!(account.is_some());

//     // the account now has additional data the size of two U8Node.
//     let account = account.unwrap();
//     assert_eq!(
//         account.data.len(),
//         TokenAccount::BASE_LEN + std::mem::size_of::<U8Node<u32, u32>>() * 2
//     );
// }
