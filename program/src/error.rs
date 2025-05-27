use pinocchio::program_error::ProgramError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SigilError {
    /// 0 - Error deserializing an account
    DeserializationError,
    /// 1 - Error serializing an account
    SerializationError,
    /// 2 - Invalid program owner
    InvalidProgramOwner,
    /// 3 - Invalid PDA derivation
    InvalidPda,
    /// 4 - Expected empty account
    ExpectedEmptyAccount,
    /// 5 - Expected non empty account
    ExpectedNonEmptyAccount,
    /// 6 - Expected signer account
    ExpectedSignerAccount,
    /// 7 - Expected writable account
    ExpectedWritableAccount,
    /// 8 - Account mismatch
    AccountMismatch,
    /// 9 - Invalid account key
    InvalidAccountKey,
    /// 10 - Numerical overflow
    NumericalOverflow,
    /// 11 - Invalid ticker
    InvalidTicker,
    /// 12 - Invalid mint
    InvalidMint,
    /// 13 - Invalid token account
    InvalidTokenAccount,
    /// 14 - Insufficient funds
    InsufficientFunds,
    /// 15 - Maximum supply reached
    MaximumSupplyReached,
    /// 16 - Maximum supply reached
    MintHasSupply,
}

impl From<SigilError> for ProgramError {
    fn from(e: SigilError) -> Self {
        Self::Custom(e as u32)
    }
}
