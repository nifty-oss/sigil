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
import { SIGIL_PROGRAM_ADDRESS } from '../programs';
import { ResolvedAccount, getAccountMetaFactory } from '../shared';

export type CreateTokenAccountInstruction<
  TProgram extends string = typeof SIGIL_PROGRAM_ADDRESS,
  TAccountTokenAccount extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TAccountUser extends string | IAccountMeta<string> = string,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountTokenAccount extends string
        ? WritableAccount<TAccountTokenAccount>
        : TAccountTokenAccount,
      TAccountAuthority extends string
        ? ReadonlyAccount<TAccountAuthority>
        : TAccountAuthority,
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

export type CreateTokenAccountInstructionData = {
  discriminator: number;
  capacity: number;
};

export type CreateTokenAccountInstructionDataArgs = { capacity: number };

export function getCreateTokenAccountInstructionDataEncoder(): Encoder<CreateTokenAccountInstructionDataArgs> {
  return mapEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['capacity', getU8Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 4 })
  );
}

export function getCreateTokenAccountInstructionDataDecoder(): Decoder<CreateTokenAccountInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['capacity', getU8Decoder()],
  ]);
}

export function getCreateTokenAccountInstructionDataCodec(): Codec<
  CreateTokenAccountInstructionDataArgs,
  CreateTokenAccountInstructionData
> {
  return combineCodec(
    getCreateTokenAccountInstructionDataEncoder(),
    getCreateTokenAccountInstructionDataDecoder()
  );
}

export type CreateTokenAccountInput<
  TAccountTokenAccount extends string = string,
  TAccountAuthority extends string = string,
  TAccountUser extends string = string,
  TAccountPayer extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The token authority account. */
  tokenAccount: Address<TAccountTokenAccount>;
  /** The authority for the token account. */
  authority: Address<TAccountAuthority>;
  /** The pubkey of the user associated with the token account */
  user: Address<TAccountUser>;
  /** The account paying for the storage fees. */
  payer: TransactionSigner<TAccountPayer>;
  /** The system program */
  systemProgram?: Address<TAccountSystemProgram>;
  capacity: CreateTokenAccountInstructionDataArgs['capacity'];
};

export function getCreateTokenAccountInstruction<
  TAccountTokenAccount extends string,
  TAccountAuthority extends string,
  TAccountUser extends string,
  TAccountPayer extends string,
  TAccountSystemProgram extends string,
>(
  input: CreateTokenAccountInput<
    TAccountTokenAccount,
    TAccountAuthority,
    TAccountUser,
    TAccountPayer,
    TAccountSystemProgram
  >
): CreateTokenAccountInstruction<
  typeof SIGIL_PROGRAM_ADDRESS,
  TAccountTokenAccount,
  TAccountAuthority,
  TAccountUser,
  TAccountPayer,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = SIGIL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    tokenAccount: { value: input.tokenAccount ?? null, isWritable: true },
    authority: { value: input.authority ?? null, isWritable: false },
    user: { value: input.user ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.tokenAccount),
      getAccountMeta(accounts.authority),
      getAccountMeta(accounts.user),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCreateTokenAccountInstructionDataEncoder().encode(
      args as CreateTokenAccountInstructionDataArgs
    ),
  } as CreateTokenAccountInstruction<
    typeof SIGIL_PROGRAM_ADDRESS,
    TAccountTokenAccount,
    TAccountAuthority,
    TAccountUser,
    TAccountPayer,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedCreateTokenAccountInstruction<
  TProgram extends string = typeof SIGIL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The token authority account. */
    tokenAccount: TAccountMetas[0];
    /** The authority for the token account. */
    authority: TAccountMetas[1];
    /** The pubkey of the user associated with the token account */
    user: TAccountMetas[2];
    /** The account paying for the storage fees. */
    payer: TAccountMetas[3];
    /** The system program */
    systemProgram: TAccountMetas[4];
  };
  data: CreateTokenAccountInstructionData;
};

export function parseCreateTokenAccountInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedCreateTokenAccountInstruction<TProgram, TAccountMetas> {
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
  return {
    programAddress: instruction.programAddress,
    accounts: {
      tokenAccount: getNextAccount(),
      authority: getNextAccount(),
      user: getNextAccount(),
      payer: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCreateTokenAccountInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
