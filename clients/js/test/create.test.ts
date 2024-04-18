// import { appendTransactionInstruction, pipe } from '@solana/web3.js';
// import test from 'ava';
// import {
//   createDefaultSolanaClient,
//   createDefaultTransaction,
//   generateKeyPairSignerWithSol,
//   signAndSendTransaction,
// } from './_setup.js';
// import { getCreateInstruction } from '../src/index.js';

// test('it creates a new token account', async (t) => {
//   // Given an authority key pair with some SOL.
//   const client = createDefaultSolanaClient();
//   const authority = await generateKeyPairSignerWithSol(client);

//   const tokenNamespace = findTokenNamespacePda({});

//   // When we create a new counter account.
//   const createIx = await getCreateInstruction({
//     authority,
//     payer: authority,
//     namespace,
//   });
//   await pipe(
//     await createDefaultTransaction(client, authority),
//     (tx) => appendTransactionInstruction(createIx, tx),
//     (tx) => signAndSendTransaction(client, tx)
//   );

//   // Then we expect the counter account to exist and have a value of 0.
//   const counter = await fetchCounterFromSeeds(client.rpc, {
//     authority: authority.address,
//   });
//   t.like(counter, <Counter>{
//     data: {
//       authority: authority.address,
//       value: 0,
//     },
//   });
// });
