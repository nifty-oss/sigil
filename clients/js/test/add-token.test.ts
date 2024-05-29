import {
  appendTransactionInstruction,
  generateKeyPairSigner,
  pipe,
} from '@solana/web3.js';
import test from 'ava';
import {
  Tag,
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

test('it can add a token to a pocket account', async (t) => {
  // Given a payer and authority key pairs.
  const client = createDefaultSolanaClient();

  const payer = await generateKeyPairSignerWithSol(client);
  const authority = await generateKeyPairSigner();

  // And we create a mint account.
  const ticker = 'USDC';

  const [mint] = await findMintPda({
    ticker: Buffer.from(ticker),
    authority: authority.address,
  });

  const createMintIx = getCreateMintInstruction({
    payer,
    mint,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker,
  });

  await pipe(
    await createDefaultTransaction(client, payer),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // And we create a new pocket account.
  const user = await generateKeyPairSigner();

  const [pocket] = await findPocketPda({
    authority: authority.address,
    user: user.address,
  });

  const createTokenAccountIx = getCreateTokenAccountInstruction({
    payer,
    user: user.address,
    authority: authority.address,
    tokenAccount: pocket,
    capacity: 100,
  });

  await pipe(
    await createDefaultTransaction(client, payer),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // When we add a new token to the pocket account.

  const addTokenIx = getAddTokenInstruction({
    mint,
    tokenAccount: pocket,
    user: user.address,
  });

  await pipe(
    await createDefaultTransaction(client, payer),
    (tx) => appendTransactionInstruction(addTokenIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  t.like(await fetchPocket(client.rpc, pocket), {
    address: pocket,
    data: {
      tag: Tag.Pocket,
      authority: authority.address,
      user: user.address,
      tokens: [{ ticker: 'USDC', amount: 0 }],
    },
  });
});

test.skip('it can add multiple tokens to a pocket account', async (t) => {
  t.timeout(80_000);
  // Given a payer and authority key pairs.
  const client = createDefaultSolanaClient();

  const payer = await generateKeyPairSignerWithSol(client);
  const authority = await generateKeyPairSigner();

  // And we create a new pocket account.
  const user = await generateKeyPairSigner();

  const [pocket] = await findPocketPda({
    authority: authority.address,
    user: user.address,
  });

  const createTokenAccountIx = getCreateTokenAccountInstruction({
    payer,
    user: user.address,
    authority: authority.address,
    tokenAccount: pocket,
    capacity: 100,
  });

  await pipe(
    await createDefaultTransaction(client, payer),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // And a list of tickers in a random order.
  const tickers = Array.from({ length: 100 }, (_, i) => `T${100 + i}`)
    .map((value) => ({ value, sort: Math.random() }))
    .sort((a, b) => a.sort - b.sort)
    .map(({ value }) => value);

  // When we add multiple tokens to the pocket account.
  for (const ticker of tickers) {
    const [mint] = await findMintPda({
      ticker: Buffer.from(ticker),
      authority: authority.address,
    });

    const createMintIx = getCreateMintInstruction({
      payer,
      mint,
      authority,
      decimals: 0,
      maxSupply: 1000,
      ticker,
    });

    const addTokenIx = getAddTokenInstruction({
      mint,
      tokenAccount: pocket,
      user: user.address,
    });

    await pipe(
      await createDefaultTransaction(client, payer),
      (tx) => appendTransactionInstruction(createMintIx, tx),
      (tx) => appendTransactionInstruction(addTokenIx, tx),
      (tx) => signAndSendTransaction(client, tx)
    );
  }

  // Then the pocket account should have all the tokens.
  const account = await fetchPocket(client.rpc, pocket);
  t.like(account, {
    address: pocket,
    data: {
      tag: Tag.Pocket,
      authority: authority.address,
      user: user.address,
    },
  });

  t.true(account.data.tokens.length === tickers.length);
});
