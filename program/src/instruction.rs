use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateArgs {
    pub capacity: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateMintArgs {
    pub ticker: String,
    pub max_supply: u64,
    pub decimals: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MintToArgs {
    pub amount: u32,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct BurnArgs {
    pub amount: u32,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct TransferArgs {
    pub amount: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instruction {
    /// Create a new mint account from a ticker and a namespace.
    /// The namespace authority must sign the transaction to sign off on creation of a new mint account
    /// in their namespace.
    #[account(0, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, writable, signer, name="namespace", desc = "The namespace for the token account.")]
    #[account(2, writable, name="mint", desc = "The mint account PDA derived from the ticker and namespace.")]
    #[account(3, name="system_program", desc = "The system program")]
    #[account(4, name="nifty_program", desc = "The Nifty Asset program")]
    CreateMint(CreateMintArgs),

    /// Create a new token account for a user in a specific namespace.
    /// This only requires a payer to sign to pay for state bond storage fees.
    /// Otherwise, anyone can permissionlessly create a token account for a user in a namespace.
    #[account(0, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, name="namespace", desc = "The namespace for the token account.")]
    #[account(2, name="user", desc = "The pubkey of the user associated with the token account")]
    #[account(3, writable, name="token_account", desc = "The token namespace account.")]
    #[account(4, name="system_program", desc = "The system program")]
    CreateTokenAccount(CreateArgs),

    /// Add a new ticker to a user's token account.
    /// This only requires a payer to sign to pay for the small additional state bond storage fees.
    #[account(0, optional, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, name="user", desc = "The pubkey of the user associated with the token account")]
    #[account(2, name="mint", desc = "The mint account for the token to be added.")]
    #[account(3, writable, name="token_account", desc = "The token namespace account.")]
    #[account(4, optional, name="system_program", desc = "The system program")]
    AddToken,

    /// Create a new mint account from a ticker and a namespace.
    /// The namespace authority must sign the transaction to sign off on creation of a new mint account
    /// in their namespace.
    #[account(0, optional, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, writable, signer, name="namespace", desc = "The namespace for the token account.")]
    #[account(2, writable, name="mint", desc = "The mint account PDA derived from the ticker and namespace.")]
    #[account(3, writable, name="token_account", desc = "The token namespace account.")]
    #[account(4, optional, name="system_program", desc = "The system program")]
    #[account(5, name="nifty_program", desc = "The Nifty Asset program")]
    MintTo(MintToArgs),

    /// Allows a user to burn tokens from their account.
    #[account(0, signer, name="user", desc = "The user of the token account")]
    #[account(1, writable, name="mint", desc = "The mint account PDA derived from the ticker and namespace.")]
    #[account(2, writable, name="token_account", desc = "The token namespace account.")]
    #[account(3, name="nifty_program", desc = "The Nifty Asset program")]
    Burn(BurnArgs),

    /// Transfer tokens from one user to another.
    #[account(0, optional, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, signer, name="user", desc = "The pubkey of the user associated with the token account")]
    #[account(2, name="recipient", desc = "The recipient account.")]
    #[account(3, name="mint", desc = "The mint account for the token to be transferred")]
    #[account(4, writable, name="user_token_account", desc = "The token namespace account.")]
    #[account(5, writable, name="recipient_token_account", desc = "The token namespace account.")]
    #[account(6, optional, name="system_program", desc = "The system program")]
    Transfer(TransferArgs),
}
