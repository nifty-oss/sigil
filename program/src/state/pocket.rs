use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use pinocchio::{program_error::ProgramError, pubkey, pubkey::Pubkey};
use stevia::{
    collections::{U16ArraySet, U16ArraySetMut},
    ZeroCopy,
};

use super::{Tag, Token};
use crate::error::SigilError;

/// Mask for the delegate count stored in the lower nibble of `Base.data[1]`.
const DELEGATE_MASK: u8 = 0x0F;
// Reserved for future use:
// const FREEZE_MASK: u8 = 0xF0;
// const FREEZE_SHIFT: u8 = 4;

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
    ///   1. bits 0-3: delegate_count, bits 4-7: reserved (future freeze_count)
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

    /// Returns the number of active delegations for this Pocket.
    pub fn delegate_count(&self) -> u8 {
        self.data[1] & DELEGATE_MASK
    }

    /// Returns `true` if this Pocket has any active delegations.
    pub fn has_delegates(&self) -> bool {
        self.data[1] & DELEGATE_MASK > 0
    }

    /// Increments the delegate count by 1. Fails if the count would exceed 15.
    pub fn increment_delegate_count(&mut self) -> Result<(), ProgramError> {
        let count = self.delegate_count();
        let new_count = count
            .checked_add(1)
            .ok_or::<ProgramError>(SigilError::NumericalOverflow.into())?;
        if new_count > DELEGATE_MASK {
            return Err(SigilError::NumericalOverflow.into());
        }
        // Clear lower nibble, write new count, preserve upper nibble.
        self.data[1] = (self.data[1] & !DELEGATE_MASK) | new_count;
        Ok(())
    }

    /// Decrements the delegate count by 1. Fails if the count is already 0.
    pub fn decrement_delegate_count(&mut self) -> Result<(), ProgramError> {
        let count = self.delegate_count();
        let new_count = count
            .checked_sub(1)
            .ok_or::<ProgramError>(SigilError::NoActiveDelegation.into())?;
        self.data[1] = (self.data[1] & !DELEGATE_MASK) | new_count;
        Ok(())
    }
}

impl ZeroCopy for Base {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a zeroed Base for testing.
    fn new_base() -> Base {
        Base::zeroed()
    }

    // -- delegate_count --

    #[test]
    fn delegate_count_zero_on_fresh_base() {
        let base = new_base();
        assert_eq!(base.delegate_count(), 0);
    }

    #[test]
    fn delegate_count_reads_lower_nibble_only() {
        let mut base = new_base();
        // Simulate upper nibble set (future freeze_count), lower nibble = 5.
        base.data[1] = 0xA5; // upper = 0xA, lower = 0x5
        assert_eq!(base.delegate_count(), 5);
    }

    // -- has_delegates --

    #[test]
    fn has_delegates_false_when_zero() {
        let base = new_base();
        assert!(!base.has_delegates());
    }

    #[test]
    fn has_delegates_true_when_nonzero() {
        let mut base = new_base();
        base.data[1] = 0x01;
        assert!(base.has_delegates());
    }

    #[test]
    fn has_delegates_false_when_only_upper_nibble_set() {
        let mut base = new_base();
        // Upper nibble set, lower nibble zero — no delegates.
        base.data[1] = 0xF0;
        assert!(!base.has_delegates());
    }

    // -- increment_delegate_count --

    #[test]
    fn increment_from_zero() {
        let mut base = new_base();
        assert!(base.increment_delegate_count().is_ok());
        assert_eq!(base.delegate_count(), 1);
        assert!(base.has_delegates());
    }

    #[test]
    fn increment_multiple_times() {
        let mut base = new_base();
        for i in 1..=10 {
            assert!(base.increment_delegate_count().is_ok());
            assert_eq!(base.delegate_count(), i);
        }
    }

    #[test]
    fn increment_to_max() {
        let mut base = new_base();
        // Fill to 15 (max for 4-bit nibble).
        for _ in 0..15 {
            assert!(base.increment_delegate_count().is_ok());
        }
        assert_eq!(base.delegate_count(), 15);
    }

    #[test]
    fn increment_overflow_at_max() {
        let mut base = new_base();
        // Fill to 15.
        for _ in 0..15 {
            base.increment_delegate_count().unwrap();
        }
        // 16th should fail.
        let result = base.increment_delegate_count();
        assert!(result.is_err());
        // Count must remain 15.
        assert_eq!(base.delegate_count(), 15);
    }

    #[test]
    fn increment_preserves_upper_nibble() {
        let mut base = new_base();
        // Set upper nibble to 0xB0 (simulate future freeze_count).
        base.data[1] = 0xB0;
        assert!(base.increment_delegate_count().is_ok());
        assert_eq!(base.delegate_count(), 1);
        // Upper nibble must be untouched.
        assert_eq!(base.data[1] & 0xF0, 0xB0);
    }

    #[test]
    fn increment_preserves_upper_nibble_through_many_ops() {
        let mut base = new_base();
        base.data[1] = 0xC0;
        for i in 1..=15 {
            base.increment_delegate_count().unwrap();
            assert_eq!(base.delegate_count(), i);
            assert_eq!(base.data[1] & 0xF0, 0xC0);
        }
    }

    // -- decrement_delegate_count --

    #[test]
    fn decrement_from_one() {
        let mut base = new_base();
        base.increment_delegate_count().unwrap();
        assert!(base.decrement_delegate_count().is_ok());
        assert_eq!(base.delegate_count(), 0);
        assert!(!base.has_delegates());
    }

    #[test]
    fn decrement_multiple_times() {
        let mut base = new_base();
        for _ in 0..5 {
            base.increment_delegate_count().unwrap();
        }
        for expected in (0..5).rev() {
            assert!(base.decrement_delegate_count().is_ok());
            assert_eq!(base.delegate_count(), expected);
        }
    }

    #[test]
    fn decrement_underflow_at_zero() {
        let mut base = new_base();
        let result = base.decrement_delegate_count();
        assert!(result.is_err());
        assert_eq!(base.delegate_count(), 0);
    }

    #[test]
    fn decrement_preserves_upper_nibble() {
        let mut base = new_base();
        base.data[1] = 0xD3; // upper = 0xD, lower = 3
        assert!(base.decrement_delegate_count().is_ok());
        assert_eq!(base.delegate_count(), 2);
        assert_eq!(base.data[1] & 0xF0, 0xD0);
    }

    // -- round-trip --

    #[test]
    fn increment_then_decrement_round_trip() {
        let mut base = new_base();
        base.data[1] = 0x70; // upper nibble preset
        for _ in 0..8 {
            base.increment_delegate_count().unwrap();
        }
        for _ in 0..8 {
            base.decrement_delegate_count().unwrap();
        }
        assert_eq!(base.delegate_count(), 0);
        assert!(!base.has_delegates());
        // Upper nibble unchanged through entire round-trip.
        assert_eq!(base.data[1], 0x70);
    }
}
