import { address, appendTransactionInstruction, pipe } from '@solana/web3.js';
import { ASSET_PROGRAM_ID } from '@nifty-oss/asset';
import test from 'ava';
import {
  getCreateMintInstruction,
  findMintAccountPda,
  getCreateTokenAccountInstruction,
  findTokenAccountPda,
  getAddTokenInstruction,
  fetchTokenAccount,
  getMintToInstruction,
} from '../src/index.js';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup.js';

test('it can mint tokens to an existing account', async (t) => {
  const client = createDefaultSolanaClient();

  const namespace = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const mintAmount = 100;

  const [mint] = await findMintAccountPda({
    ticker,
    namespace: namespace.address,
  });

  const createMintIx = getCreateMintInstruction({
    payer: namespace,
    mint,
    namespace,
    niftyProgram: address(ASSET_PROGRAM_ID),
    decimals: 0,
    maxSupply: 1000,
    ticker: 'USDC',
  });

  const [tokenAccount] = await findTokenAccountPda({
    namespace: namespace.address,
    user: user.address,
  });

  const createTokenAccountIx = getCreateTokenAccountInstruction({
    payer: namespace,
    user: user.address,
    namespace: namespace.address,
    tokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const addTokenIx = getAddTokenInstruction({
    payer: user,
    user: user.address,
    mint,
    tokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(addTokenIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  let account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.tree.nodes[0].ticker === ticker);
  t.assert(account?.data.tree.nodes[0].amount === 0);

  const mintToIx = getMintToInstruction({
    payer: namespace,
    namespace,
    mint,
    tokenAccount,
    amount: mintAmount,
    niftyProgram: address(ASSET_PROGRAM_ID),
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(mintToIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.tree.nodes[0].ticker === ticker);
  t.assert(account?.data.tree.nodes[0].amount === mintAmount);
});

test('it can add a token and mint to it account', async (t) => {
  const client = createDefaultSolanaClient();

  const namespace = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const mintAmount = 100;

  const [mint] = await findMintAccountPda({
    ticker,
    namespace: namespace.address,
  });

  const createMintIx = getCreateMintInstruction({
    payer: namespace,
    mint,
    namespace,
    niftyProgram: address(ASSET_PROGRAM_ID),
    decimals: 0,
    maxSupply: 1000,
    ticker: 'USDC',
  });

  const [tokenAccount] = await findTokenAccountPda({
    namespace: namespace.address,
    user: user.address,
  });

  const createTokenAccountIx = getCreateTokenAccountInstruction({
    payer: namespace,
    user: user.address,
    namespace: namespace.address,
    tokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // We do not add the token prior to minting. The mint instruction should
  // add it for us.

  const mintToIx = getMintToInstruction({
    payer: namespace,
    namespace,
    mint,
    tokenAccount,
    amount: mintAmount,
    niftyProgram: address(ASSET_PROGRAM_ID),
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(mintToIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.tree.nodes[0].ticker === ticker);
  t.assert(account?.data.tree.nodes[0].amount === mintAmount);
});
