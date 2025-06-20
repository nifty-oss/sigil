use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use pinocchio::{pubkey, pubkey::Pubkey};
use stevia::{
    collections::{U16ArraySet, U16ArraySetMut},
    ZeroCopy,
};

use super::{Tag, Token};

/// Struct representing an account storing tokens.
///
/// A `Pocket` account is a PDA with the seeds `["pocket", <authority>, <user>]`.
pub struct Pocket<'a> {
    /// Base account data.
    pub base: &'a Base,

    /// Tokens stored in the account.
    pub tokens: U16ArraySet<'a, Token>,
}

impl<'a> Pocket<'a> {
    // Base + u32 array set.
    pub const LEN: usize = Base::LEN + size_of::<u16>();

    pub const PREFIX: &'static [u8] = b"pocket";

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let (base, tokens) = bytes.split_at(Base::LEN);

        let base = Base::load(base);
        let tokens = U16ArraySet::from_bytes(tokens);

        Self { base, tokens }
    }

    pub fn find_pda(seeds: &PocketSeeds) -> (Pubkey, u8) {
        pubkey::find_program_address(
            &[Self::PREFIX, seeds.user.as_ref(), seeds.authority.as_ref()],
            &crate::ID,
        )
    }
}

pub struct PocketSeeds {
    /// Account authority.
    pub authority: Pubkey,

    /// Account owner.
    pub user: Pubkey,
}

/// Mutable version of the `Pocket` struct.
pub struct PocketMut<'a> {
    /// Base account data.
    pub base: &'a mut Base,

    /// Tokens stored in the account.
    pub tokens: U16ArraySetMut<'a, Token>,
}

impl<'a> PocketMut<'a> {
    pub fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (base, tokens) = bytes.split_at_mut(Base::LEN);

        let base = Base::load_mut(base);
        let tokens = U16ArraySetMut::from_bytes_mut(tokens);

        Self { base, tokens }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Base {
    /// Internal data.
    ///   0. tag
    ///   1. not in use
    data: [u8; 2],

    /// Authority of the account.
    pub authority: Pubkey,

    /// Owner of the account.
    pub user: Pubkey,
}

impl Base {
    /// Bytes required to store a `Base`.
    pub const LEN: usize = std::mem::size_of::<Base>();

    pub fn tag(&self) -> Tag {
        Tag::from(self.data[0])
    }

    pub fn set_tag(&mut self, tag: Tag) {
        self.data[0] = tag.into();
    }
}

impl ZeroCopy for Base {}
