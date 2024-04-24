import { address, appendTransactionInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import {
  fetchTokenAccount,
  findMintPda,
  findTokenAccountPda,
  getAddTokenInstruction,
  getCreateMintInstruction,
  getCreateTokenAccountInstruction,
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

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const mintAmount = 100;

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

  const addTokenIx = getAddTokenInstruction({
    payer: user,
    user: user.address,
    mint,
    tokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(addTokenIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  let account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.tree.nodes[0].ticker === ticker);
  t.assert(account?.data.tree.nodes[0].amount === 0);

  const mintToIx = getMintToInstruction({
    payer: authority,
    authority,
    mint,
    tokenAccount,
    amount: mintAmount,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(mintToIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.tree.nodes[0].ticker === ticker);
  t.assert(account?.data.tree.nodes[0].amount === mintAmount);
});

test('it can add a token and mint to it account', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const mintAmount = 100;

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

  // We do not need to add the token prior to minting. The mint instruction should
  // add it for us.

  const mintToIx = getMintToInstruction({
    payer: authority,
    authority,
    mint,
    tokenAccount,
    amount: mintAmount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(mintToIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const account = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(account?.data.tree.nodes[0].ticker === ticker);
  t.assert(account?.data.tree.nodes[0].amount === mintAmount);
});
