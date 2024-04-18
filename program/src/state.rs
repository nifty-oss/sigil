use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;
use stevia::collections::{AVLTree, AVLTreeMut};

pub const KEY_SIZE: usize = std::mem::size_of::<u64>();

// Allocator size for the AVL tree.
const AVL_SIZE: usize = std::mem::size_of::<u32>() * 6;

#[repr(u64)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Key {
    #[default]
    Uninitialized,
    TokenAccount,
}

// Can't derive Pod/Zeroable for enums, so we have to do it ourselves manually.
// Our enum is aligned to 8 bytes, so we can safely use the unsafe impls.
unsafe impl Zeroable for Key {}
unsafe impl Pod for Key {}

pub type Ticker = [u8; 4];
pub type Amount = u32;

pub struct TokenSeeds {
    pub user: Pubkey,
    pub namespace: Pubkey,
}

/// Seeds: "token_account", <user>, <namespace>
pub struct TokenAccount<'a> {
    pub header: &'a Header,

    pub tokens: AVLTree<'a, Ticker, Amount>,
}

/// Seeds: "token_account", <user>, <namespace>
pub struct TokenAccountMut<'a> {
    pub header: &'a mut Header,

    pub tokens: AVLTreeMut<'a, Ticker, Amount>,
}

impl<'a> TokenAccount<'a> {
    pub const LEN: usize = Header::LEN + AVL_SIZE;

    pub const PREFIX: &'static [u8] = b"token_account";

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let (header, data) = bytes.split_at(Header::LEN);
        let header = Header::from_bytes(header);

        let tokens = AVLTree::from_bytes(data);

        Self { header, tokens }
    }

    pub fn find_pda(seeds: TokenSeeds) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[Self::PREFIX, seeds.user.as_ref(), seeds.namespace.as_ref()],
            &crate::ID,
        )
    }
}

impl<'a> TokenAccountMut<'a> {
    pub fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (header, data) = bytes.split_at_mut(Header::LEN);
        let header = Header::from_bytes_mut(header);

        let tokens = AVLTreeMut::from_bytes_mut(data);

        Self { header, tokens }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Header {
    pub key: Key,
    pub namespace: Pubkey,
}

impl Header {
    /// Bytes required to store an `Header`.
    pub const LEN: usize = std::mem::size_of::<Header>();

    pub fn from_bytes(bytes: &'_ [u8]) -> &'_ Self {
        bytemuck::from_bytes::<Header>(bytes)
    }

    pub fn from_bytes_mut(bytes: &'_ mut [u8]) -> &'_ mut Self {
        bytemuck::from_bytes_mut::<Header>(bytes)
    }
}

// TODO: ByteMuck? Only 21 bytes.
#[derive(Clone, Debug, Default, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MintMetadata {
    pub ticker: String,
    pub supply: u64,
    pub max_supply: u64,
    pub decimals: u8,
}

impl MintMetadata {
    pub const LEN: usize = std::mem::size_of::<MintMetadata>();
}
