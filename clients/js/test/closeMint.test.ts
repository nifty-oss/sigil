import {
  SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM,
  appendTransactionInstruction,
  isSolanaError,
  pipe,
} from '@solana/web3.js';
import test from 'ava';
import {
  fetchTokenAccount,
  findTokenAccountPda,
  getBurnInstruction,
  getCloseMintInstruction,
} from '../src/index.js';
import { setupAndMint } from './_common.js';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup.js';

test('it can close a mint account w/ 0 supply', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const mintAmount = 100;
  const burnAmount = 100;

  const mint = await setupAndMint(client, authority, user, ticker, mintAmount);

  const [tokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: user.address,
  });

  let tokenAccountData = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(tokenAccountData?.data.tree.nodes[0].amount === mintAmount);

  const burnIx = getBurnInstruction({
    user,
    mint,
    tokenAccount,
    amount: burnAmount,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(burnIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  tokenAccountData = await fetchTokenAccount(client.rpc, tokenAccount);

  // No supply left.
  t.assert(tokenAccountData?.data.tree.nodes[0].amount === 0);

  const closeMintIx = getCloseMintInstruction({
    mint,
    authority,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(closeMintIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );
});

test('it cannot close a mint account w/ remaining supply', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const mintAmount = 100;
  const burnAmount = 25;

  const mint = await setupAndMint(client, authority, user, ticker, mintAmount);

  const [tokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: user.address,
  });

  let tokenAccountData = await fetchTokenAccount(client.rpc, tokenAccount);

  t.assert(tokenAccountData?.data.tree.nodes[0].amount === mintAmount);

  const burnIx = getBurnInstruction({
    user,
    mint,
    tokenAccount,
    amount: burnAmount,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(burnIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  tokenAccountData = await fetchTokenAccount(client.rpc, tokenAccount);

  // Supply left.
  t.assert(tokenAccountData?.data.tree.nodes[0].amount > 0);

  const closeMintIx = getCloseMintInstruction({
    mint,
    authority,
  });

  const promise = pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(closeMintIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  const error = await t.throwsAsync<Error & { data: { logs: string[] } }>(
    promise
  );

  // MintHasSupply
  const code = 16;

  if (isSolanaError(error.cause, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM)) {
    t.assert(
      error.cause.context.code === code,
      `expected error code ${code}, received ${error.cause.context.code}`
    );
  } else {
    t.fail("expected a custom error, but didn't get one");
  }
});
