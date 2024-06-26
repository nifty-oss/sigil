use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum SigilError {
    /// 0 - Error deserializing an account
    #[error("Error deserializing an account")]
    DeserializationError,
    /// 1 - Error serializing an account
    #[error("Error serializing an account")]
    SerializationError,
    /// 2 - Invalid program owner
    #[error("Invalid program owner. This likely mean the provided account does not exist")]
    InvalidProgramOwner,
    /// 3 - Invalid PDA derivation
    #[error("Invalid PDA derivation")]
    InvalidPda,
    /// 4 - Expected empty account
    #[error("Expected empty account")]
    ExpectedEmptyAccount,
    /// 5 - Expected non empty account
    #[error("Expected non empty account")]
    ExpectedNonEmptyAccount,
    /// 6 - Expected signer account
    #[error("Expected signer account")]
    ExpectedSignerAccount,
    /// 7 - Expected writable account
    #[error("Expected writable account")]
    ExpectedWritableAccount,
    /// 8 - Account mismatch
    #[error("Account mismatch")]
    AccountMismatch,
    /// 9 - Invalid account key
    #[error("Invalid account key")]
    InvalidAccountKey,
    /// 10 - Numerical overflow
    #[error("Numerical overflow")]
    NumericalOverflow,
    /// 11 - Invalid ticker
    #[error("Invalid utf8 ticker")]
    InvalidTicker,
    /// 12 - Invalid mint
    #[error("Invalid mint")]
    InvalidMint,
    /// 13 - Invalid token account
    #[error("Invalid token account")]
    InvalidTokenAccount,
    /// 14 - Insufficient funds
    #[error("Insufficient funds")]
    InsufficientFunds,
    /// 15 - Maximum supply reached
    #[error("Maximum supply reached")]
    MaximumSupplyReached,
    /// 16 - Maximum supply reached
    #[error("Cannot close mint account with supply")]
    MintHasSupply,
}

impl PrintProgramError for SigilError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<SigilError> for ProgramError {
    fn from(e: SigilError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SigilError {
    fn type_of() -> &'static str {
        "Mpl Project Name Error"
    }
}
