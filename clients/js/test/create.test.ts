import { address, appendTransactionInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import {
  fetchTokenAccount,
  findMintPda,
  findTokenAccountPda,
  getAddTokenInstruction,
  getCreateMintInstruction,
  getCreateTokenAccountInstruction,
} from '../src/index.js';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup.js';

test('it can create a new mint account', async (t) => {
  // Given an authority key pair with some SOL.
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';

  const [mint] = await findMintPda({
    ticker: Array.from(ticker).map((c) => c.charCodeAt(0)),
    authority: authority.address,
  });

  const createMintIx = getCreateMintInstruction({
    payer: authority,
    mint,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker: 'USDC',
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  t.pass();
});

test('it can create a new token account', async (t) => {
  // Given an authority key pair with some SOL.
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';

  const [mint] = await findMintPda({
    ticker: Array.from(ticker).map((c) => c.charCodeAt(0)),
    authority: authority.address,
  });

  // When we create a new counter account.
  const createMintIx = getCreateMintInstruction({
    payer: authority,
    mint,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker: 'USDC',
  });

  const [tokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: user.address,
  });

  const createTokenAccountIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: user.address,
    authority: authority.address,
    tokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.authority === authority.address);
  t.assert(account?.data.user === user.address);
});

test('it can add tokens to a token account', async (t) => {
  // Given an authority key pair with some SOL.
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker1 = 'USDC';
  const ticker2 = 'BONK';

  const [mint1] = await findMintPda({
    ticker: Array.from(ticker1).map((c) => c.charCodeAt(0)),
    authority: authority.address,
  });

  const [mint2] = await findMintPda({
    ticker: Array.from(ticker2).map((c) => c.charCodeAt(0)),
    authority: authority.address,
  });

  const createMintIx1 = getCreateMintInstruction({
    payer: authority,
    mint: mint1,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker: ticker1,
  });

  const createMintIx2 = getCreateMintInstruction({
    payer: authority,
    mint: mint2,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker: ticker2,
  });

  const [tokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: user.address,
  });

  const createTokenAccountIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: user.address,
    authority: authority.address,
    tokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createMintIx1, tx),
    (tx) => appendTransactionInstruction(createMintIx2, tx),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const addTokenIx1 = getAddTokenInstruction({
    payer: user,
    user: user.address,
    mint: mint1,
    tokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(addTokenIx1, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const addTokenIx2 = getAddTokenInstruction({
    payer: user,
    user: user.address,
    mint: mint2,
    tokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(addTokenIx2, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.tree.nodes.length === 2);
  t.assert(account?.data.tree.nodes[0].ticker === ticker1);
  t.assert(account?.data.tree.nodes[1].ticker === ticker2);
});
