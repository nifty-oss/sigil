use bytemuck::{Pod, Zeroable};
use pinocchio::{pubkey, pubkey::Pubkey};
use stevia::ZeroCopy;

use super::{Tag, Ticker};

/// Delegation data.
///
/// A `Delegation` is a PDA with the seeds `["delegate", <user>, <authority>, <ticker>]`.
/// It represents a per-ticker delegate authorization for a user's Pocket account.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Delegation {
    /// Internal data.
    ///   0. tag
    ///   1. bump
    ///   2. ticker
    ///   3. ticker
    ///   4. ticker
    ///   5. ticker
    ///   6. not in use
    ///   7. not in use
    data: [u8; 8],

    /// The delegate pubkey authorized to act on behalf of the user.
    pub delegate: Pubkey,

    /// The user who granted the delegation.
    pub user: Pubkey,

    /// The authority namespace this delegation applies to.
    pub authority: Pubkey,
}

impl Delegation {
    /// Length of the delegation data.
    pub const LEN: usize = std::mem::size_of::<Delegation>();

    /// Prefix for the delegation PDA seeds.
    pub const PREFIX: &'static [u8] = b"delegate";

    pub fn tag(&self) -> Tag {
        Tag::from(self.data[0])
    }

    pub fn bump(&self) -> u8 {
        self.data[1]
    }

    pub fn ticker(&self) -> Ticker {
        self.data[2..6].try_into().unwrap()
    }

    pub fn set_tag(&mut self, tag: Tag) {
        self.data[0] = tag.into();
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.data[1] = bump;
    }

    pub fn set_ticker(&mut self, ticker: Ticker) {
        self.data[2..6].copy_from_slice(&ticker);
    }

    pub fn find_pda(seeds: &DelegationSeeds) -> (Pubkey, u8) {
        pubkey::find_program_address(
            &[
                Self::PREFIX,
                seeds.user.as_ref(),
                seeds.authority.as_ref(),
                seeds.ticker,
            ],
            &crate::ID,
        )
    }
}

/// Default implementation for zero-copy trait.
impl ZeroCopy for Delegation {}

/// Seeds for the delegation PDA.
pub struct DelegationSeeds<'a> {
    /// The user who grants the delegation.
    pub user: Pubkey,

    /// The authority namespace.
    pub authority: Pubkey,

    /// The ticker this delegation applies to.
    pub ticker: &'a Ticker,
}
