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
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountUser extends string | IAccountMeta<string> = string,
  TAccountMint extends string | IAccountMeta<string> = string,
  TAccountTokenAccount extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountUser extends string
        ? ReadonlyAccount<TAccountUser>
        : TAccountUser,
      TAccountMint extends string
        ? ReadonlyAccount<TAccountMint>
        : TAccountMint,
      TAccountTokenAccount extends string
        ? WritableAccount<TAccountTokenAccount>
        : TAccountTokenAccount,
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
    (value) => ({ ...value, discriminator: 2 })
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
  TAccountPayer extends string = string,
  TAccountUser extends string = string,
  TAccountMint extends string = string,
  TAccountTokenAccount extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The account paying for the storage fees. */
  payer?: TransactionSigner<TAccountPayer>;
  /** The pubkey of the user associated with the token account */
  user: Address<TAccountUser>;
  /** The mint account for the token to be added. */
  mint: Address<TAccountMint>;
  /** The token namespace account. */
  tokenAccount: Address<TAccountTokenAccount>;
  /** The system program */
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getAddTokenInstruction<
  TAccountPayer extends string,
  TAccountUser extends string,
  TAccountMint extends string,
  TAccountTokenAccount extends string,
  TAccountSystemProgram extends string,
>(
  input: AddTokenInput<
    TAccountPayer,
    TAccountUser,
    TAccountMint,
    TAccountTokenAccount,
    TAccountSystemProgram
  >
): AddTokenInstruction<
  typeof TOKEN_LITE_PROGRAM_ADDRESS,
  TAccountPayer,
  TAccountUser,
  TAccountMint,
  TAccountTokenAccount,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = TOKEN_LITE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    payer: { value: input.payer ?? null, isWritable: true },
    user: { value: input.user ?? null, isWritable: false },
    mint: { value: input.mint ?? null, isWritable: false },
    tokenAccount: { value: input.tokenAccount ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.user),
      getAccountMeta(accounts.mint),
      getAccountMeta(accounts.tokenAccount),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getAddTokenInstructionDataEncoder().encode({}),
  } as AddTokenInstruction<
    typeof TOKEN_LITE_PROGRAM_ADDRESS,
    TAccountPayer,
    TAccountUser,
    TAccountMint,
    TAccountTokenAccount,
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
    /** The account paying for the storage fees. */
    payer?: TAccountMetas[0] | undefined;
    /** The pubkey of the user associated with the token account */
    user: TAccountMetas[1];
    /** The mint account for the token to be added. */
    mint: TAccountMetas[2];
    /** The token namespace account. */
    tokenAccount: TAccountMetas[3];
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
      payer: getNextOptionalAccount(),
      user: getNextAccount(),
      mint: getNextAccount(),
      tokenAccount: getNextAccount(),
      systemProgram: getNextOptionalAccount(),
    },
    data: getAddTokenInstructionDataDecoder().decode(instruction.data),
  };
}
