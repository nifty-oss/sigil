/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  mapEncoder,
} from '@solana/codecs';
import {
  IAccountMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
  ReadonlyAccount,
  WritableAccount,
  WritableSignerAccount,
} from '@solana/instructions';
import { IAccountSignerMeta, TransactionSigner } from '@solana/signers';
import { TOKEN_LITE_PROGRAM_ADDRESS } from '../programs';
import { ResolvedAccount, getAccountMetaFactory } from '../shared';

export type AddTokenInstruction<
  TProgram extends string = typeof TOKEN_LITE_PROGRAM_ADDRESS,
  TAccountTokenAccount extends string | IAccountMeta<string> = string,
  TAccountMint extends string | IAccountMeta<string> = string,
  TAccountUser extends string | IAccountMeta<string> = string,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountTokenAccount extends string
        ? WritableAccount<TAccountTokenAccount>
        : TAccountTokenAccount,
      TAccountMint extends string
        ? ReadonlyAccount<TAccountMint>
        : TAccountMint,
      TAccountUser extends string
        ? ReadonlyAccount<TAccountUser>
        : TAccountUser,
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type AddTokenInstructionData = { discriminator: number };

export type AddTokenInstructionDataArgs = {};

export function getAddTokenInstructionDataEncoder(): Encoder<AddTokenInstructionDataArgs> {
  return mapEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: 0 })
  );
}

export function getAddTokenInstructionDataDecoder(): Decoder<AddTokenInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getAddTokenInstructionDataCodec(): Codec<
  AddTokenInstructionDataArgs,
  AddTokenInstructionData
> {
  return combineCodec(
    getAddTokenInstructionDataEncoder(),
    getAddTokenInstructionDataDecoder()
  );
}

export type AddTokenInput<
  TAccountTokenAccount extends string = string,
  TAccountMint extends string = string,
  TAccountUser extends string = string,
  TAccountPayer extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The token authority account. */
  tokenAccount: Address<TAccountTokenAccount>;
  /** The mint account for the token to be added. */
  mint: Address<TAccountMint>;
  /** The pubkey of the user associated with the token account */
  user: Address<TAccountUser>;
  /** The account paying for the storage fees. */
  payer?: TransactionSigner<TAccountPayer>;
  /** The system program */
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getAddTokenInstruction<
  TAccountTokenAccount extends string,
  TAccountMint extends string,
  TAccountUser extends string,
  TAccountPayer extends string,
  TAccountSystemProgram extends string,
>(
  input: AddTokenInput<
    TAccountTokenAccount,
    TAccountMint,
    TAccountUser,
    TAccountPayer,
    TAccountSystemProgram
  >
): AddTokenInstruction<
  typeof TOKEN_LITE_PROGRAM_ADDRESS,
  TAccountTokenAccount,
  TAccountMint,
  TAccountUser,
  TAccountPayer,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = TOKEN_LITE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    tokenAccount: { value: input.tokenAccount ?? null, isWritable: true },
    mint: { value: input.mint ?? null, isWritable: false },
    user: { value: input.user ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.tokenAccount),
      getAccountMeta(accounts.mint),
      getAccountMeta(accounts.user),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getAddTokenInstructionDataEncoder().encode({}),
  } as AddTokenInstruction<
    typeof TOKEN_LITE_PROGRAM_ADDRESS,
    TAccountTokenAccount,
    TAccountMint,
    TAccountUser,
    TAccountPayer,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedAddTokenInstruction<
  TProgram extends string = typeof TOKEN_LITE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The token authority account. */
    tokenAccount: TAccountMetas[0];
    /** The mint account for the token to be added. */
    mint: TAccountMetas[1];
    /** The pubkey of the user associated with the token account */
    user: TAccountMetas[2];
    /** The account paying for the storage fees. */
    payer?: TAccountMetas[3] | undefined;
    /** The system program */
    systemProgram?: TAccountMetas[4] | undefined;
  };
  data: AddTokenInstructionData;
};

export function parseAddTokenInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAddTokenInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  const getNextOptionalAccount = () => {
    const accountMeta = getNextAccount();
    return accountMeta.address === TOKEN_LITE_PROGRAM_ADDRESS
      ? undefined
      : accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      tokenAccount: getNextAccount(),
      mint: getNextAccount(),
      user: getNextAccount(),
      payer: getNextOptionalAccount(),
      systemProgram: getNextOptionalAccount(),
    },
    data: getAddTokenInstructionDataDecoder().decode(instruction.data),
  };
}
