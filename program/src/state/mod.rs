mod mint;
mod pocket;

pub use mint::*;
pub use pocket::*;

use bytemuck::{Pod, Zeroable};
use num_enum::{FromPrimitive, IntoPrimitive};

/// Account tag.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum Tag {
    #[default]
    Uninitialized,
    Mint,
    Pocket,
}

/// Struct representing a token entry in an account.
#[repr(C)]
#[derive(Clone, Copy, Default, Eq, Pod, Zeroable)]
pub struct Token {
    pub ticker: Ticker,
    pub amount: u32,
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ticker.cmp(&other.ticker)
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.ticker == other.ticker
    }
}

/// Type representing a mint ticker.
pub type Ticker = [u8; 4];

impl From<Ticker> for Token {
    fn from(ticker: Ticker) -> Self {
        Self { ticker, amount: 0 }
    }
}
