import { appendTransactionInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import {
  fetchTokenAccount,
  findTokenAccountPda,
  getBurnInstruction,
} from '../src/index.js';
import { setupAndMint } from './_common.js';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup.js';

test('it can burn tokens', async (t) => {
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

  t.assert(
    tokenAccountData?.data.tree.nodes[0].amount === mintAmount - burnAmount
  );
});
