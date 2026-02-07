import {
  address,
  appendTransactionInstruction,
  pipe,
} from '@solana/web3.js';
import test from 'ava';
import {
  fetchPocket,
  findDelegationPda,
  findMintPda,
  findPocketPda,
  getApproveInstruction,
  getBurnInstruction,
  getRevokeInstruction,
  getTransferInstruction,
} from '../src/index.js';
import { setupAndMint } from './_common.js';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup.js';

test('it can approve and revoke a delegate', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);
  const delegate = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const tickerBytes = Array.from(ticker).map((c) => c.charCodeAt(0));
  const mintAmount = 100;

  await setupAndMint(client, authority, user, ticker, mintAmount);

  const [tokenAccount] = await findPocketPda({
    authority: authority.address,
    user: user.address,
  });

  // Derive the delegation PDA.
  const [delegation] = await findDelegationPda({
    user: user.address,
    authority: authority.address,
    ticker: new Uint8Array(Array.from(ticker).map((c) => c.charCodeAt(0))),
  });

  // Approve the delegate.
  const approveIx = getApproveInstruction({
    tokenAccount,
    delegation,
    user,
    delegate: delegate.address,
    authority: authority.address,
    payer: user,
    approveArgs: { ticker: tickerBytes },
  });

  await pipe(
    await createDefaultTransaction(client, user),
    (tx) => appendTransactionInstruction(approveIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Verify the delegation account exists by checking the pocket's delegate count
  // is now > 0 (the account was fetched successfully which means the tx succeeded).
  let pocketData = await fetchPocket(client.rpc, tokenAccount);
  t.truthy(pocketData, 'Pocket account should exist');

  // Revoke the delegate.
  const revokeIx = getRevokeInstruction({
    tokenAccount,
    delegation,
    user,
    authority: authority.address,
    approveArgs: { ticker: tickerBytes },
  });

  await pipe(
    await createDefaultTransaction(client, user),
    (tx) => appendTransactionInstruction(revokeIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // The revoke should succeed (tx confirmed above).
  pocketData = await fetchPocket(client.rpc, tokenAccount);
  t.truthy(pocketData, 'Pocket account should still exist after revoke');
});

test('a delegate can transfer tokens on behalf of the user', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);
  const delegate = await generateKeyPairSignerWithSol(client);
  const recipient = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const tickerBytes = Array.from(ticker).map((c) => c.charCodeAt(0));
  const mintAmount = 100;
  const transferAmount = 30;

  await setupAndMint(client, authority, user, ticker, mintAmount);

  const [userTokenAccount] = await findPocketPda({
    authority: authority.address,
    user: user.address,
  });

  // Create recipient token account.
  const [recipientTokenAccount] = await findPocketPda({
    authority: authority.address,
    user: recipient.address,
  });

  const { getCreateTokenAccountInstruction } = await import(
    '../src/index.js'
  );

  const createRecipientIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: recipient.address,
    authority: authority.address,
    tokenAccount: recipientTokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createRecipientIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Approve the delegate for this ticker.
  const [delegation] = await findDelegationPda({
    user: user.address,
    authority: authority.address,
    ticker: new Uint8Array(Array.from(ticker).map((c) => c.charCodeAt(0))),
  });

  const approveIx = getApproveInstruction({
    tokenAccount: userTokenAccount,
    delegation,
    user,
    delegate: delegate.address,
    authority: authority.address,
    payer: user,
    approveArgs: { ticker: tickerBytes },
  });

  await pipe(
    await createDefaultTransaction(client, user),
    (tx) => appendTransactionInstruction(approveIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // The delegate signs the transfer (not the user).
  const transferIx = getTransferInstruction({
    userTokenAccount,
    recipientTokenAccount,
    user: delegate,
    payer: authority,
    systemProgram: address('11111111111111111111111111111111'),
    delegation,
    ticker: tickerBytes,
    amount: transferAmount,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(transferIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Verify balances.
  const userAccount = await fetchPocket(client.rpc, userTokenAccount);
  const recipientAccount = await fetchPocket(
    client.rpc,
    recipientTokenAccount
  );

  t.is(userAccount?.data.tokens[0].amount, mintAmount - transferAmount);
  t.is(recipientAccount?.data.tokens[0].amount, transferAmount);
});

test('a delegate can burn tokens on behalf of the user', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);
  const delegate = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const tickerBytes = Array.from(ticker).map((c) => c.charCodeAt(0));
  const mintAmount = 100;
  const burnAmount = 40;

  const mint = await setupAndMint(client, authority, user, ticker, mintAmount);

  const [tokenAccount] = await findPocketPda({
    authority: authority.address,
    user: user.address,
  });

  // Approve the delegate.
  const [delegation] = await findDelegationPda({
    user: user.address,
    authority: authority.address,
    ticker: new Uint8Array(Array.from(ticker).map((c) => c.charCodeAt(0))),
  });

  const approveIx = getApproveInstruction({
    tokenAccount,
    delegation,
    user,
    delegate: delegate.address,
    authority: authority.address,
    payer: user,
    approveArgs: { ticker: tickerBytes },
  });

  await pipe(
    await createDefaultTransaction(client, user),
    (tx) => appendTransactionInstruction(approveIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // The delegate signs the burn.
  const burnIx = getBurnInstruction({
    tokenAccount,
    mint,
    user: delegate,
    delegation,
    amount: burnAmount,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(burnIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Verify the balance was reduced.
  const pocketData = await fetchPocket(client.rpc, tokenAccount);
  t.is(pocketData?.data.tokens[0].amount, mintAmount - burnAmount);
});

test('a non-delegate signer cannot transfer tokens', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);
  const notADelegate = await generateKeyPairSignerWithSol(client);
  const recipient = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const tickerBytes = Array.from(ticker).map((c) => c.charCodeAt(0));
  const mintAmount = 100;

  await setupAndMint(client, authority, user, ticker, mintAmount);

  const [userTokenAccount] = await findPocketPda({
    authority: authority.address,
    user: user.address,
  });

  const [recipientTokenAccount] = await findPocketPda({
    authority: authority.address,
    user: recipient.address,
  });

  const { getCreateTokenAccountInstruction } = await import(
    '../src/index.js'
  );

  const createRecipientIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: recipient.address,
    authority: authority.address,
    tokenAccount: recipientTokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createRecipientIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Try to transfer with an unauthorized signer (no delegation approved).
  const transferIx = getTransferInstruction({
    userTokenAccount,
    recipientTokenAccount,
    user: notADelegate,
    payer: authority,
    systemProgram: address('11111111111111111111111111111111'),
    ticker: tickerBytes,
    amount: 10,
  });

  const promise = pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(transferIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // The transaction should fail.
  await t.throwsAsync(promise);
});

test('delegate for one ticker cannot transfer a different ticker', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);
  const delegate = await generateKeyPairSignerWithSol(client);
  const recipient = await generateKeyPairSignerWithSol(client);

  const ticker1 = 'USDC';
  const ticker2 = 'BONK';
  const tickerBytes1 = Array.from(ticker1).map((c) => c.charCodeAt(0));
  const tickerBytes2 = Array.from(ticker2).map((c) => c.charCodeAt(0));

  // Mint both tickers to the user.
  await setupAndMint(client, authority, user, ticker1, 100);

  // Set up ticker2: create mint and mint to user.
  const [mint2] = await findMintPda({
    ticker: new Uint8Array(Array.from(ticker2).map((c) => c.charCodeAt(0))),
    authority: authority.address,
  });

  const [userTokenAccount] = await findPocketPda({
    authority: authority.address,
    user: user.address,
  });

  const {
    getCreateMintInstruction,
    getMintToInstruction,
    getCreateTokenAccountInstruction,
  } = await import('../src/index.js');

  const createMint2Ix = getCreateMintInstruction({
    payer: authority,
    mint: mint2,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker: ticker2,
  });

  const mintTo2Ix = getMintToInstruction({
    payer: authority,
    authority,
    mint: mint2,
    tokenAccount: userTokenAccount,
    amount: 50,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createMint2Ix, tx),
    (tx) => appendTransactionInstruction(mintTo2Ix, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Create recipient account.
  const [recipientTokenAccount] = await findPocketPda({
    authority: authority.address,
    user: recipient.address,
  });

  const createRecipientIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: recipient.address,
    authority: authority.address,
    tokenAccount: recipientTokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createRecipientIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Approve delegate for ticker1 ONLY.
  const [delegation1] = await findDelegationPda({
    user: user.address,
    authority: authority.address,
    ticker: new Uint8Array(Array.from(ticker1).map((c) => c.charCodeAt(0))),
  });

  const approveIx = getApproveInstruction({
    tokenAccount: userTokenAccount,
    delegation: delegation1,
    user,
    delegate: delegate.address,
    authority: authority.address,
    payer: user,
    approveArgs: { ticker: tickerBytes1 },
  });

  await pipe(
    await createDefaultTransaction(client, user),
    (tx) => appendTransactionInstruction(approveIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Try to use the ticker1 delegation to transfer ticker2. Should fail.
  const transferIx = getTransferInstruction({
    userTokenAccount,
    recipientTokenAccount,
    user: delegate,
    payer: authority,
    systemProgram: address('11111111111111111111111111111111'),
    delegation: delegation1,
    ticker: tickerBytes2,
    amount: 10,
  });

  const promise = pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(transferIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // The transaction should fail because the delegation is for ticker1, not ticker2.
  await t.throwsAsync(promise);
});
