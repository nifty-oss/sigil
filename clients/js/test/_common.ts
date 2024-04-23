import {
  TransactionSigner,
  address,
  appendTransactionInstruction,
  pipe,
} from '@solana/web3.js';
import { ASSET_PROGRAM_ID } from '@nifty-oss/asset';
import {
  getCreateMintInstruction,
  findMintAccountPda,
  getCreateTokenAccountInstruction,
  findTokenAccountPda,
  getMintToInstruction,
} from '../src/index.js';
import {
  Client,
  createDefaultTransaction,
  signAndSendTransaction,
} from './_setup.js';

export const setupAndMint = async (
  client: Client,
  namespace: TransactionSigner,
  user: TransactionSigner,
  ticker: string,
  amount: number
) => {
  const [mint] = await findMintAccountPda({
    ticker,
    namespace: namespace.address,
  });

  const createMintIx = getCreateMintInstruction({
    payer: namespace,
    mint,
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

  const mintToIx = getMintToInstruction({
    payer: namespace,
    namespace,
    mint,
    tokenAccount,
    amount,
    systemProgram: address('11111111111111111111111111111111'),
    niftyProgram: address(ASSET_PROGRAM_ID),
  });

  await pipe(
    await createDefaultTransaction(client, namespace),
    (tx) => appendTransactionInstruction(createMintIx, tx),
    (tx) => appendTransactionInstruction(createTokenAccountIx, tx),
    (tx) => appendTransactionInstruction(mintToIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  return mint;
};
