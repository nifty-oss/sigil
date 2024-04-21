import { address, appendTransactionInstruction, pipe } from '@solana/web3.js';
import { ASSET_PROGRAM_ID } from '@nifty-oss/asset';
import test from 'ava';
import {
  getCreateMintInstruction,
  findMintAccountPda,
  getCreateTokenAccountInstruction,
  findTokenAccountPda,
  getAddTokenInstruction,
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

  const namespace = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';

  const [mintAccount] = await findMintAccountPda({
    ticker,
    namespace: namespace.address,
  });

  // When we create a new counter account.
  const createMintIx = getCreateMintInstruction({
    payer: namespace,
    mintAccount,
    namespace,
    niftyProgram: address(ASSET_PROGRAM_ID),
    decimals: 0,
    maxSupply: 1000,
    ticker: 'USDC',
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  t.pass();
});

test('it can create a new token account', async (t) => {
  // Given an authority key pair with some SOL.
  const client = createDefaultSolanaClient();

  const namespace = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';

  const [mintAccount] = await findMintAccountPda({
    ticker,
    namespace: namespace.address,
  });

  // When we create a new counter account.
  const createMintIx = getCreateMintInstruction({
    payer: namespace,
    mintAccount,
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

  // const account = await fetchTokenAccount(client, tokenAccount);

  // t.assert(account?.data.namespace === namespace.address);
  // t.assert(account?.data.user === user.address);
  t.pass();
});

test('it can add tokens to a token account', async (t) => {
  // Given an authority key pair with some SOL.
  const client = createDefaultSolanaClient();

  const namespace = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker1 = 'USDC';
  const ticker2 = 'BONK';

  const [mintAccount1] = await findMintAccountPda({
    ticker: ticker1,
    namespace: namespace.address,
  });

  const [mintAccount2] = await findMintAccountPda({
    ticker: ticker2,
    namespace: namespace.address,
  });

  const createMintIx1 = getCreateMintInstruction({
    payer: namespace,
    mintAccount: mintAccount1,
    namespace,
    niftyProgram: address(ASSET_PROGRAM_ID),
    decimals: 0,
    maxSupply: 1000,
    ticker: 'USDC',
  });

  const createMintIx2 = getCreateMintInstruction({
    payer: namespace,
    mintAccount: mintAccount2,
    namespace,
    niftyProgram: address(ASSET_PROGRAM_ID),
    decimals: 0,
    maxSupply: 1000,
    ticker: 'BONK',
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
    (tx) => appendTransactionInstruction(createMintIx1, tx),
    (tx) => appendTransactionInstruction(createMintIx2, tx),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  t.pass();

  const addTokenIx1 = getAddTokenInstruction({
    payer: user,
    user: user.address,
    mint: mintAccount1,
    tokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(addTokenIx1, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const addTokenIx2 = getAddTokenInstruction({
    payer: user,
    user: user.address,
    mint: mintAccount2,
    tokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(addTokenIx2, tx),
    (tx) => signAndSendTransaction(client, tx)
  );
});
