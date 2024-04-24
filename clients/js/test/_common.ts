import {
  TransactionSigner,
  address,
  appendTransactionInstruction,
  getStringEncoder,
  pipe,
} from '@solana/web3.js';
import {
  findMintPda,
  findTokenAccountPda,
  getCreateMintInstruction,
  getCreateTokenAccountInstruction,
  getMintToInstruction,
} from '../src/index.js';
import {
  Client,
  createDefaultTransaction,
  signAndSendTransaction,
} from './_setup.js';

export const setupAndMint = async (
  client: Client,
  authority: TransactionSigner,
  user: TransactionSigner,
  ticker: string,
  amount: number
) => {
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

  const mintToIx = getMintToInstruction({
    payer: authority,
    authority,
    mint,
    tokenAccount,
    amount,
    systemProgram: address('11111111111111111111111111111111'),
  });

  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => appendTransactionInstruction(mintToIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  return mint;
};
