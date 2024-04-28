import { address, appendTransactionInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import {
  fetchTokenAccount,
  findMintPda,
  findTokenAccountPda,
  getCreateMintInstruction,
  getCreateTokenAccountInstruction,
  getMintToInstruction,
  getTransferInstruction,
} from '../src/index.js';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup.js';

test('it can transfer tokens', async (t) => {
  const client = createDefaultSolanaClient();

  const authority = await generateKeyPairSignerWithSol(client);
  const user = await generateKeyPairSignerWithSol(client);

  const recipient = await generateKeyPairSignerWithSol(client);

  const ticker = 'USDC';
  const mintAmount = 100;
  const transferAmount = 25;

  const [mint] = await findMintPda({
    ticker: Buffer.from(ticker),
    authority: authority.address,
  });

  const [userTokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: user.address,
  });

  const [recipientTokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: recipient.address,
  });

  // Create the mint account.
  const createMintIx = getCreateMintInstruction({
    payer: authority,
    mint,
    authority,
    decimals: 0,
    maxSupply: 1000,
    ticker: 'USDC',
  });

  // Create the user token accounts.
  const createUserTokenAccountIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: user.address,
    authority: authority.address,
    tokenAccount: userTokenAccount,
    capacity: 0,
  });
  const createRecipientTokenAccountIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: recipient.address,
    authority: authority.address,
    tokenAccount: recipientTokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => appendTransactionInstruction(createUserTokenAccountIx, tx),
    (tx) => appendTransactionInstruction(createRecipientTokenAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Mint some tokens to the user account. The ticker is added to the user account.
  const mintToIx = getMintToInstruction({
    payer: authority,
    authority,
    mint,
    tokenAccount: userTokenAccount,
    amount: mintAmount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(mintToIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  let userAccount = await fetchTokenAccount(client.rpc, userTokenAccount);
  let recipientAccount = await fetchTokenAccount(
    client.rpc,
    recipientTokenAccount
  );

  t.assert(userAccount?.data.tree.nodes[0].amount === mintAmount);
  // No token added yet.
  t.assert(recipientAccount?.data.tree.nodes.length === 0);

  const transferIx = getTransferInstruction({
    payer: authority,
    user,
    userTokenAccount,
    recipientTokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
    ticker: Array.from(ticker).map((c) => c.charCodeAt(0)),
    amount: transferAmount,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(transferIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  userAccount = await fetchTokenAccount(client.rpc, userTokenAccount);
  recipientAccount = await fetchTokenAccount(client.rpc, recipientTokenAccount);

  t.assert(
    userAccount?.data.tree.nodes[0].amount === mintAmount - transferAmount
  );
  t.assert(recipientAccount?.data.tree.nodes[0].amount === transferAmount);

  const transferBackIx = getTransferInstruction({
    payer: authority,
    user: recipient,
    userTokenAccount: recipientTokenAccount,
    recipientTokenAccount: userTokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
    ticker: Array.from(ticker).map((c) => c.charCodeAt(0)),
    amount: transferAmount,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(transferBackIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );
});
