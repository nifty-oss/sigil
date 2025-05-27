use {
    borsh::{BorshDeserialize, BorshSerialize},
    shank::ShankInstruction,
};

// #[repr(u8)]
// pub enum SigilInstruction {
//     AddToken,
//     Burn,
//     CloseMint,
//     CreateMint,
//     CreateTokenAccount,
//     MintTo,
//     Transfer,
// }

// impl TryFrom<&u8> for SigilInstruction {
//     type Error = ProgramError;

//     fn try_from(instruction: &u8) -> Result<Self, Self::Error> {
//         match instruction {
//             0 => Ok(SigilInstruction::AddToken),
//             1 => Ok(SigilInstruction::Burn),
//             2 => Ok(SigilInstruction::CloseMint),
//             3 => Ok(SigilInstruction::CreateMint),
//             4 => Ok(SigilInstruction::CreateTokenAccount),
//             5 => Ok(SigilInstruction::MintTo),
//             6 => Ok(SigilInstruction::Transfer),
//             _ => Err(ProgramError::InvalidInstructionData),
//         }
//     }
// }

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
    pub ticker: [u8; 4],
    pub amount: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankInstruction)]
    #[rustfmt::skip]
    pub enum SigilInstruction {
        /// Add a new ticker to a user's token account.
        /// This only requires a payer to sign to pay for the small additional state bond storage fees.
        #[account(0, writable, name="token_account", desc = "The token authority account.")]
        #[account(1, name="mint", desc = "The mint account for the token to be added.")]
        #[account(2, name="user", desc = "The pubkey of the user associated with the token account")]
        #[account(3, optional, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
        #[account(4, optional, name="system_program", desc = "The system program")]
        AddToken,

        /// Allows a user to burn tokens from their account.
        #[account(0, writable, name="token_account", desc = "The token authority account.")]
        #[account(1, writable, name="mint", desc = "The mint account PDA derived from the ticker and authority.")]
        #[account(2, signer, name="user", desc = "The user of the token account")]
        Burn(BurnArgs),

        /// Create a new mint account from a ticker and an authority.
        /// The authority must sign the transaction to sign off on minting new tokens.
        #[account(0, writable, name="mint", desc = "The mint account PDA derived from the ticker and authority.")]
        #[account(1, writable, signer, name="authority", desc = "The authority for the mint.")]
        #[account(2, optional, writable, signer, name="recipient", desc = "The account receiving refunded rent SOL.")]
        CloseMint,

        /// Create a new mint account from a ticker and a authority.
        /// The authority authority must sign the transaction to sign off on creation of a new mint account
        /// in their authority.
        #[account(0, writable, name="mint", desc = "The mint account PDA derived from the ticker and authority.")]
        #[account(1, writable, signer, name="authority", desc = "The authority for the token account.")]
        #[account(2, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
        #[account(3, name="system_program", desc = "The system program")]
        CreateMint(CreateMintArgs),

        /// Create a new token account for a user in a specific authority.
        /// This only requires a payer to sign to pay for state bond storage fees.
        /// Otherwise, anyone can permissionlessly create a token account for a user in a authority.
        #[account(0, writable, name="token_account", desc = "The token authority account.")]
        #[account(1, name="authority", desc = "The authority for the token account.")]
        #[account(2, name="user", desc = "The pubkey of the user associated with the token account")]
        #[account(3, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
        #[account(4, name="system_program", desc = "The system program")]
        CreateTokenAccount(CreateArgs),

        /// Create a new mint account from a ticker and an authority.
        /// The authority must sign the transaction to sign off on minting new tokens.
        #[account(0, writable, name="token_account", desc = "The token authority account.")]
        #[account(1, writable, name="mint", desc = "The mint account PDA derived from the ticker and authority.")]
        #[account(2, writable, signer, name="authority", desc = "The authority for the mint.")]
        #[account(3, optional, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
        #[account(4, optional, name="system_program", desc = "The system program")]
        MintTo(MintToArgs),

        /// Transfer tokens from one user to another.
        #[account(0, writable, name="user_token_account", desc = "The token authority account.")]
        #[account(1, writable, name="recipient_token_account", desc = "The token authority account.")]
        #[account(2, signer, name="user", desc = "The pubkey of the user associated with the token account")]
        #[account(3, optional, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
        #[account(4, optional, name="system_program", desc = "The system program")]
        Transfer(TransferArgs),
    }
