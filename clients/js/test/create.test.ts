import { address, appendTransactionInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import {
  Tag,
  fetchMint,
  fetchPocket,
  findMintPda,
  findPocketPda,
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
    ticker: Buffer.from(ticker),
    authority: authority.address,
  });

  const createMintIx = getCreateMintInstruction({
    payer: authority,
    mint,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const account = await fetchMint(client.rpc, mint);

  t.assert(account?.data.tag == Tag.Mint);
  t.assert(account?.data.decimals === 0);
  t.assert(account?.data.ticker === ticker);
  t.assert(account?.data.authority === authority.address);
  t.assert(account?.data.supply === 0n);
  t.assert(account?.data.maxSupply === 1000n);
});

test('it can create a new token account', async (t) => {
  // Given an authority key pair with some SOL.
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';

  const [mint] = await findMintPda({
    ticker: Buffer.from(ticker),
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

  const [tokenAccount] = await findPocketPda({
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

  const account = await fetchPocket(client.rpc, tokenAccount);

  t.assert(account?.data.tag == Tag.Pocket);
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
    ticker: Buffer.from(ticker1),
    authority: authority.address,
  });

  const [mint2] = await findMintPda({
    ticker: Buffer.from(ticker2),
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

  const [tokenAccount] = await findPocketPda({
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

  const account = await fetchPocket(client.rpc, tokenAccount);

  t.assert(account?.data.tokens.length === 2);
  t.assert(account?.data.tokens[0].ticker === ticker2);
  t.assert(account?.data.tokens[1].ticker === ticker1);
});
