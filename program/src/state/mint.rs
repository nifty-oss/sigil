use bytemuck::{Pod, Zeroable};
use pinocchio::{pubkey, pubkey::Pubkey};
use stevia::ZeroCopy;

use super::{Tag, Ticker};

/// Mint data.
///
/// The mint is a PDA with the seeds `["mint", <authority>, <ticker>]`.
#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct Mint {
    /// Internal data.
    ///   0. tag
    ///   1. bump
    ///   2. decimals
    ///   3. not in use
    ///   4-7. ticker
    data: [u8; 8],

    /// Authority of the mint.
    pub authority: Pubkey,

    /// Current supply of the mint.
    pub supply: u64,

    /// Maximum supply of the mint.
    pub max_supply: u64,
}

impl Mint {
    /// Length of the mint data.
    pub const LEN: usize = std::mem::size_of::<Mint>();

    /// Prefix for the mint PDA seeds.
    pub const PREFIX: &'static [u8] = b"mint";

    pub fn tag(&self) -> Tag {
        Tag::from(self.data[0])
    }

    pub fn bump(&self) -> u8 {
        self.data[1]
    }

    pub fn decimals(&self) -> u8 {
        self.data[2]
    }

    pub fn ticker(&self) -> Ticker {
        self.data[4..8].try_into().unwrap()
    }

    pub fn set_tag(&mut self, tag: Tag) {
        self.data[0] = tag.into();
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.data[1] = bump;
    }

    pub fn set_decimals(&mut self, decimals: u8) {
        self.data[2] = decimals;
    }

    pub fn set_ticker(&mut self, ticker: Ticker) {
        self.data[4..8].copy_from_slice(&ticker);
    }

    pub fn find_pda(seeds: &MintSeeds) -> (Pubkey, u8) {
        pubkey::find_program_address(
            &[Self::PREFIX, seeds.authority.as_ref(), seeds.ticker],
            &crate::ID,
        )
    }
}

/// Default implementation for zero-copy trait.
impl ZeroCopy for Mint {}

/// Seeds for the mint PDA.
pub struct MintSeeds<'a> {
    /// Mint ticker.
    pub ticker: &'a Ticker,

    /// Mint authority.
    pub authority: Pubkey,
}
