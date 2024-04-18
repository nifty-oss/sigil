/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import { getU8Encoder } from '@solana/codecs';
import { Program, ProgramWithErrors } from '@solana/programs';
import {
  TokenLiteProgramError,
  TokenLiteProgramErrorCode,
  getTokenLiteProgramErrorFromCode,
} from '../errors';
import {
  ParsedAddTokenInstruction,
  ParsedCreateMintInstruction,
  ParsedCreateTokenAccountInstruction,
} from '../instructions';
import { memcmp } from '../shared';

export const TOKEN_LITE_PROGRAM_ADDRESS =
  'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9' as Address<'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9'>;

export type TokenLiteProgram =
  Program<'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9'> &
    ProgramWithErrors<TokenLiteProgramErrorCode, TokenLiteProgramError>;

export function getTokenLiteProgram(): TokenLiteProgram {
  return {
    name: 'tokenLite',
    address: TOKEN_LITE_PROGRAM_ADDRESS,
    getErrorFromCode(code: TokenLiteProgramErrorCode, cause?: Error) {
      return getTokenLiteProgramErrorFromCode(code, cause);
    },
  };
}

export enum TokenLiteInstruction {
  CreateMint,
  CreateTokenAccount,
  AddToken,
}

export function identifyTokenLiteInstruction(
  instruction: { data: Uint8Array } | Uint8Array
): TokenLiteInstruction {
  const data =
    instruction instanceof Uint8Array ? instruction : instruction.data;
  if (memcmp(data, getU8Encoder().encode(0), 0)) {
    return TokenLiteInstruction.CreateMint;
  }
  if (memcmp(data, getU8Encoder().encode(1), 0)) {
    return TokenLiteInstruction.CreateTokenAccount;
  }
  if (memcmp(data, getU8Encoder().encode(2), 0)) {
    return TokenLiteInstruction.AddToken;
  }
  throw new Error(
    'The provided instruction could not be identified as a tokenLite instruction.'
  );
}

export type ParsedTokenLiteInstruction<
  TProgram extends string = 'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9',
> =
  | ({
      instructionType: TokenLiteInstruction.CreateMint;
    } & ParsedCreateMintInstruction<TProgram>)
  | ({
      instructionType: TokenLiteInstruction.CreateTokenAccount;
    } & ParsedCreateTokenAccountInstruction<TProgram>)
  | ({
      instructionType: TokenLiteInstruction.AddToken;
    } & ParsedAddTokenInstruction<TProgram>);
