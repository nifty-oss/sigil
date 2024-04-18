use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateArgs {
    pub capacity: u32,
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
pub struct AddTokenArgs {
    pub ticker: String,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instruction {
    /// Create a new mint account from a ticker and a namespace.
    /// The namespace authority must sign the transaction to sign off on creation of a new mint account
    /// in their namespace.
    #[account(0, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, writable, signer, name="namespace", desc = "The namespace for the token account.")]
    #[account(2, writable, name="mint_account", desc = "The mint account PDA derived from the ticker and namespace.")]
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
    #[account(1, name="namespace", desc = "The namespace for the token account.")]
    #[account(2, name="user", desc = "The pubkey of the user associated with the token account")]
    #[account(3, writable, name="token_account", desc = "The token namespace account.")]
    #[account(4, optional, name="system_program", desc = "The system program")]
    AddToken(AddTokenArgs),
}
