use bytemuck::{Pod, Zeroable};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use solana_program::pubkey::Pubkey;
use stevia::{
    collections::{u8_avl_tree::U8Allocator, U8AVLTree, U8AVLTreeMut},
    ZeroCopy,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum Tag {
    #[default]
    Uninitialized,
    Mint,
    TokenAccount,
}

pub type Ticker = [u8; 4];
pub type Amount = u32;

pub struct TokenSeeds {
    pub user: Pubkey,
    pub authority: Pubkey,
}

pub struct MintSeeds<'a> {
    pub ticker: &'a [u8; 4],
    pub authority: Pubkey,
}

/// Seeds: "token_account", <user>, <authority>
pub struct TokenAccount<'a> {
    pub header: &'a Header,

    pub tokens: U8AVLTree<'a, Ticker, Amount>,
}

/// Seeds: "token_account", <user>, <authority>
pub struct TokenAccountMut<'a> {
    pub header: &'a mut Header,

    pub tokens: U8AVLTreeMut<'a, Ticker, Amount>,
}

impl<'a> TokenAccount<'a> {
    // Header + AVL tree allocator without any nodes.
    pub const BASE_LEN: usize = Header::LEN + std::mem::size_of::<U8Allocator>();

    pub const PREFIX: &'static [u8] = b"token_account";

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let (header, data) = bytes.split_at(Header::LEN);
        let header = Header::load(header);

        let tokens = U8AVLTree::from_bytes(data);

        Self { header, tokens }
    }

    pub fn find_pda(seeds: &TokenSeeds) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[Self::PREFIX, seeds.user.as_ref(), seeds.authority.as_ref()],
            &crate::ID,
        )
    }
}

impl<'a> TokenAccountMut<'a> {
    pub fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (header, data) = bytes.split_at_mut(Header::LEN);
        let header = Header::load_mut(header);

        let tokens = U8AVLTreeMut::from_bytes_mut(data);

        Self { header, tokens }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Header {
    // Tag, empty x 3 bytes.
    data: [u8; 4],
    pub authority: Pubkey,
    pub user: Pubkey,
}

impl ZeroCopy for Header {}

impl Header {
    /// Bytes required to store an `Header`.
    pub const LEN: usize = std::mem::size_of::<Header>();

    pub fn tag(&self) -> Tag {
        Tag::try_from(self.data[0]).unwrap()
    }

    pub fn set_tag(&mut self, tag: Tag) {
        self.data[0] = tag.into();
    }
}

/// Seeds: "mint", <ticker>, <authority>
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Mint {
    // Tag, bump, decimals, empty.
    data: [u8; 8],
    pub authority: Pubkey,
    pub supply: u64,
    pub max_supply: u64,
}

impl Mint {
    pub const LEN: usize = std::mem::size_of::<Mint>();
    pub const PREFIX: &'static [u8] = b"mint";

    pub fn tag(&self) -> Tag {
        Tag::try_from(self.data[0]).unwrap()
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
        *self.data.get_mut(0).unwrap() = tag.into();
    }

    pub fn set_bump(&mut self, bump: u8) {
        *self.data.get_mut(1).unwrap() = bump;
    }

    pub fn set_decimals(&mut self, decimals: u8) {
        *self.data.get_mut(2).unwrap() = decimals;
    }

    pub fn set_ticker(&mut self, ticker: Ticker) {
        self.data[4..8].copy_from_slice(&ticker);
    }

    pub fn find_pda(seeds: &MintSeeds) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::PREFIX,
                seeds.ticker.as_ref(),
                seeds.authority.as_ref(),
            ],
            &crate::ID,
        )
    }
}

/// Default implementation for zero-copy trait.
impl ZeroCopy for Mint {}
