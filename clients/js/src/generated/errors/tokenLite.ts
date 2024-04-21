/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

export const enum TokenLiteProgramErrorCode {
  /** DeserializationError: Error deserializing an account */
  DESERIALIZATION_ERROR = 0x0, // 0
  /** SerializationError: Error serializing an account */
  SERIALIZATION_ERROR = 0x1, // 1
  /** InvalidProgramOwner: Invalid program owner. This likely mean the provided account does not exist */
  INVALID_PROGRAM_OWNER = 0x2, // 2
  /** InvalidPda: Invalid PDA derivation */
  INVALID_PDA = 0x3, // 3
  /** ExpectedEmptyAccount: Expected empty account */
  EXPECTED_EMPTY_ACCOUNT = 0x4, // 4
  /** ExpectedNonEmptyAccount: Expected non empty account */
  EXPECTED_NON_EMPTY_ACCOUNT = 0x5, // 5
  /** ExpectedSignerAccount: Expected signer account */
  EXPECTED_SIGNER_ACCOUNT = 0x6, // 6
  /** ExpectedWritableAccount: Expected writable account */
  EXPECTED_WRITABLE_ACCOUNT = 0x7, // 7
  /** AccountMismatch: Account mismatch */
  ACCOUNT_MISMATCH = 0x8, // 8
  /** InvalidAccountKey: Invalid account key */
  INVALID_ACCOUNT_KEY = 0x9, // 9
  /** NumericalOverflow: Numerical overflow */
  NUMERICAL_OVERFLOW = 0xa, // 10
  /** InvalidTicker: Invalid utf8 ticker */
  INVALID_TICKER = 0xb, // 11
  /** InvalidMint: Invalid mint */
  INVALID_MINT = 0xc, // 12
  /** InvalidTokenAccount: Invalid token account */
  INVALID_TOKEN_ACCOUNT = 0xd, // 13
  /** InsufficientFunds: Insufficient funds */
  INSUFFICIENT_FUNDS = 0xe, // 14
  /** MaximumSupplyReached: Maximum supply reached */
  MAXIMUM_SUPPLY_REACHED = 0xf, // 15
}

export class TokenLiteProgramError extends Error {
  override readonly name = 'TokenLiteProgramError';

  readonly code: TokenLiteProgramErrorCode;

  readonly cause: Error | undefined;

  constructor(
    code: TokenLiteProgramErrorCode,
    name: string,
    message: string,
    cause?: Error
  ) {
    super(`${name} (${code}): ${message}`);
    this.code = code;
    this.cause = cause;
  }
}

let tokenLiteProgramErrorCodeMap:
  | Record<TokenLiteProgramErrorCode, [string, string]>
  | undefined;
if (__DEV__) {
  tokenLiteProgramErrorCodeMap = {
    [TokenLiteProgramErrorCode.DESERIALIZATION_ERROR]: [
      'DeserializationError',
      `Error deserializing an account`,
    ],
    [TokenLiteProgramErrorCode.SERIALIZATION_ERROR]: [
      'SerializationError',
      `Error serializing an account`,
    ],
    [TokenLiteProgramErrorCode.INVALID_PROGRAM_OWNER]: [
      'InvalidProgramOwner',
      `Invalid program owner. This likely mean the provided account does not exist`,
    ],
    [TokenLiteProgramErrorCode.INVALID_PDA]: [
      'InvalidPda',
      `Invalid PDA derivation`,
    ],
    [TokenLiteProgramErrorCode.EXPECTED_EMPTY_ACCOUNT]: [
      'ExpectedEmptyAccount',
      `Expected empty account`,
    ],
    [TokenLiteProgramErrorCode.EXPECTED_NON_EMPTY_ACCOUNT]: [
      'ExpectedNonEmptyAccount',
      `Expected non empty account`,
    ],
    [TokenLiteProgramErrorCode.EXPECTED_SIGNER_ACCOUNT]: [
      'ExpectedSignerAccount',
      `Expected signer account`,
    ],
    [TokenLiteProgramErrorCode.EXPECTED_WRITABLE_ACCOUNT]: [
      'ExpectedWritableAccount',
      `Expected writable account`,
    ],
    [TokenLiteProgramErrorCode.ACCOUNT_MISMATCH]: [
      'AccountMismatch',
      `Account mismatch`,
    ],
    [TokenLiteProgramErrorCode.INVALID_ACCOUNT_KEY]: [
      'InvalidAccountKey',
      `Invalid account key`,
    ],
    [TokenLiteProgramErrorCode.NUMERICAL_OVERFLOW]: [
      'NumericalOverflow',
      `Numerical overflow`,
    ],
    [TokenLiteProgramErrorCode.INVALID_TICKER]: [
      'InvalidTicker',
      `Invalid utf8 ticker`,
    ],
    [TokenLiteProgramErrorCode.INVALID_MINT]: ['InvalidMint', `Invalid mint`],
    [TokenLiteProgramErrorCode.INVALID_TOKEN_ACCOUNT]: [
      'InvalidTokenAccount',
      `Invalid token account`,
    ],
    [TokenLiteProgramErrorCode.INSUFFICIENT_FUNDS]: [
      'InsufficientFunds',
      `Insufficient funds`,
    ],
    [TokenLiteProgramErrorCode.MAXIMUM_SUPPLY_REACHED]: [
      'MaximumSupplyReached',
      `Maximum supply reached`,
    ],
  };
}

export function getTokenLiteProgramErrorFromCode(
  code: TokenLiteProgramErrorCode,
  cause?: Error
): TokenLiteProgramError {
  if (__DEV__) {
    return new TokenLiteProgramError(
      code,
      ...(
        tokenLiteProgramErrorCodeMap as Record<
          TokenLiteProgramErrorCode,
          [string, string]
        >
      )[code],
      cause
    );
  }

  return new TokenLiteProgramError(
    code,
    'Unknown',
    'Error message not available in production bundles. Compile with __DEV__ set to true to see more information.',
    cause
  );
}
