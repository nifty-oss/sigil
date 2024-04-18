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
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instruction {
    #[account(0, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, writable, signer, name="namespace", desc = "The namespace for the token account.")]
    #[account(2, name="user", desc = "The pubkey of the user associated with the token account")]
    #[account(3, writable, name="token_account", desc = "The token namespace account.")]
    #[account(4, name="system_program", desc = "The system program")]
    CreateTokenAccount(CreateArgs),

    #[account(0, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(1, writable, signer, name="namespace", desc = "The namespace for the token account.")]
    #[account(2, writable, name="mint_account", desc = "The token namespace account.")]
    #[account(3, name="system_program", desc = "The system program")]
    #[account(4, name="nifty_program", desc = "The Nifty Asset program")]
    CreateMint(CreateMintArgs),

}
