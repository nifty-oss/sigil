import { address, appendTransactionInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import {
  fetchTokenAccount,
  findTokenAccountPda,
  getCreateTokenAccountInstruction,
  getTransferInstruction,
} from '../src/index.js';
import { setupAndMint } from './_common.js';
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

  const mint = await setupAndMint(client, authority, user, ticker, mintAmount);

  const [userTokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: user.address,
  });

  const [recipientTokenAccount] = await findTokenAccountPda({
    authority: authority.address,
    user: recipient.address,
  });

  // Create recipient token account.
  const createTokenAccountIx = getCreateTokenAccountInstruction({
    payer: authority,
    user: recipient.address,
    authority: authority.address,
    tokenAccount: recipientTokenAccount,
    capacity: 0,
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
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
    recipient: recipient.address,
    mint,
    userTokenAccount,
    recipientTokenAccount,
    systemProgram: address('11111111111111111111111111111111'),
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
});
