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

export type CloseMintInstruction<
  TProgram extends string = typeof TOKEN_LITE_PROGRAM_ADDRESS,
  TAccountMint extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TAccountRecipient extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountMint extends string
        ? WritableAccount<TAccountMint>
        : TAccountMint,
      TAccountAuthority extends string
        ? WritableSignerAccount<TAccountAuthority> &
            IAccountSignerMeta<TAccountAuthority>
        : TAccountAuthority,
      TAccountRecipient extends string
        ? WritableSignerAccount<TAccountRecipient> &
            IAccountSignerMeta<TAccountRecipient>
        : TAccountRecipient,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type CloseMintInstructionData = { discriminator: number };

export type CloseMintInstructionDataArgs = {};

export function getCloseMintInstructionDataEncoder(): Encoder<CloseMintInstructionDataArgs> {
  return mapEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: 2 })
  );
}

export function getCloseMintInstructionDataDecoder(): Decoder<CloseMintInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getCloseMintInstructionDataCodec(): Codec<
  CloseMintInstructionDataArgs,
  CloseMintInstructionData
> {
  return combineCodec(
    getCloseMintInstructionDataEncoder(),
    getCloseMintInstructionDataDecoder()
  );
}

export type CloseMintInput<
  TAccountMint extends string = string,
  TAccountAuthority extends string = string,
  TAccountRecipient extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The mint account PDA derived from the ticker and authority. */
  mint: Address<TAccountMint>;
  /** The authority for the mint. */
  authority: TransactionSigner<TAccountAuthority>;
  /** The account receiving refunded rent SOL. */
  recipient?: TransactionSigner<TAccountRecipient>;
  /** The system program */
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getCloseMintInstruction<
  TAccountMint extends string,
  TAccountAuthority extends string,
  TAccountRecipient extends string,
  TAccountSystemProgram extends string,
>(
  input: CloseMintInput<
    TAccountMint,
    TAccountAuthority,
    TAccountRecipient,
    TAccountSystemProgram
  >
): CloseMintInstruction<
  typeof TOKEN_LITE_PROGRAM_ADDRESS,
  TAccountMint,
  TAccountAuthority,
  TAccountRecipient,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = TOKEN_LITE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    mint: { value: input.mint ?? null, isWritable: true },
    authority: { value: input.authority ?? null, isWritable: true },
    recipient: { value: input.recipient ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.mint),
      getAccountMeta(accounts.authority),
      getAccountMeta(accounts.recipient),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCloseMintInstructionDataEncoder().encode({}),
  } as CloseMintInstruction<
    typeof TOKEN_LITE_PROGRAM_ADDRESS,
    TAccountMint,
    TAccountAuthority,
    TAccountRecipient,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedCloseMintInstruction<
  TProgram extends string = typeof TOKEN_LITE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The mint account PDA derived from the ticker and authority. */
    mint: TAccountMetas[0];
    /** The authority for the mint. */
    authority: TAccountMetas[1];
    /** The account receiving refunded rent SOL. */
    recipient?: TAccountMetas[2] | undefined;
    /** The system program */
    systemProgram: TAccountMetas[3];
  };
  data: CloseMintInstructionData;
};

export function parseCloseMintInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedCloseMintInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 4) {
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
      mint: getNextAccount(),
      authority: getNextAccount(),
      recipient: getNextOptionalAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCloseMintInstructionDataDecoder().decode(instruction.data),
  };
}