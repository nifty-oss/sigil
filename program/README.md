# Sigil

A simple token program that minimizes on-chain state bond costs to store token accounts.

## Overview

Sigil is a new fungible item specification and program on Solana that represents fungible items on-chain in an absolutely minimal amount of data to keep data storage costs as cheap as possible. Building the spec to use on-chain data has significant benefits over using off-chain approaches which might be even cheaper, e.g. only  Merkle tree proofs stored on-chain in that transactions are small and do not require passing in cumbersome proofs and token data is directly accessible by other Solana programs via account state. Token Lite therefore represents the optimal trade-off in on-chain, accessible data with as minimal costs as possible given the current limitations of account data on Solana's runtime.

The specification is not designed to replace the existing token programs on Solana, which are well-established, and more fully featured, but rather designed to allow capturing new use-cases for which the current standards are prohibitively expensive. A motivating example is that of gaming, where a game studio may wish to create many assets for users, subsidizing their rent costs to reduce friction, but where current costs for creating new token accounts for each asset could become prohibitively expensive given a large set of users and multiple assets per user. 

The program is implemented with all data structs using zero-copy bytemuck implementations to make the program extremely efficient in terms of compute and memory usage.

## Design

The specification is currently represented entirely by two types of accounts: Mints and Token Accounts. Mint accounts uniquely define a type of fungible item and encode the authority and supply data in account state, while the mint name is encoded via the PDA derivation. Token Accounts are defined *per user* and contain pairs of mint tickers and amounts to encode the user's ownership amounts of various assets.

### Mint

Mint accounts are PDAs derived from the seeds "mint", the four character ticker, e.g. "USDC", and the authority of the mint. The authority acts as a namespace for tickers to prevent squatting on valuable tickers that would inevitably happen if tickers were globally namespaced.

The on-chain Mint struct is shown below.

```rust
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
```

### Token Account

Token Accounts are PDAs derived from the seeds "token_account", and the the user and authority pubkeys and are defined per-user to allow efficient storing of mint and amount pairs, but are also namespaced by the authority of the mint. Each token account has a header which stores the account tag as well as the authority and user pubkeys to allow for efficient indexing. Ticker and Amount pairs are stored in an on-chain AVL tree which allows looking up amounts by the mint ticker. The innovation here is that creating a new user token accounts requires paying the header rent cost of 68 bytes only once for each user in a given namespace, but adding a new mint and amount pair only costs an additional 12 bytes: four for the AVL tree pointers, four for the mint ticker and four to represent a u32 amount. This is approximately 25x savings when compared to the cost of creating a new SPL token account for each new user and mint.

```rust
/// Seeds: "token_account", <user>, <authority>
pub struct TokenAccount<'a> {
    pub header: &'a Header,

    pub tokens: U8AVLTree<'a, Ticker, Amount>,
}
```

```rust
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Header {
    // Tag, empty x 3 bytes.
    data: [u8; 4],
    pub authority: Pubkey,
    pub user: Pubkey,
}
```

## Cost Savings

In the SPL token program the mint account is 82 bytes in size, plus the standard account info overhead of 128 bytes, but only has to be created once per asset, so typically represents a fixed up-front cost that is paid initially but does not scale up by number of users. Token Lite's mint account is not much smaller, but does save a few bytes coming in at 58 bytes.

Token accounts however have significant savings, as SPL token accounts require a new token account per user and mint which is 128 bytes + 165 bytes for a total of 293 bytes. In Token Lite, there is a fixed cost of 128 bytes + 68 bytes for a new user token account, but then each additional asset only requires 12 bytes without having to pay for the 128 account info header each time as the pairs are simply stored in the AVL tree on the same account. 

**Base Comparisons**

| Account           | data SIZE (Bytes) | Rent Cost @ $200 SOL |
| ----------------- | ----------------- | -------------------- |
| SPL Mint          | 82                | $0.29                |
| TL Mint           | 58                | $0.26                |
| SPL Token Account | 165               | $0.41                |
| TL Token Account  | 80                | $0.29                |



**User w/ 100 Assets**

| Account           | data SIZE (Bytes) | Rent Cost @ $200 SOL |
| ----------------- | ----------------- | -------------------- |
| SPL Token Account | 16,500            | $41                  |
| TL Token Account  | 1268              | $1.94                |



**1000 Users w/ 100 Assets**

| Account           | data SIZE (Bytes) | Rent Cost @ $200 SOL |
| ----------------- | ----------------- | -------------------- |
| SPL Token Account | 16,500,000        | $41,000              |
| TL Token Account  | 1,268,000         | $1,940               |



### Limitations

To save size, the AVL tree pointers are stored as u8s which means that each AVL tree can only store 255 mint/supply pairs. This is expected to be sufficient for most use-cases as users typically do not have more than a few hundred game assets or fungible tokens per wallet. However, given Solana accounts can be up to 10 MB in size, the amount stored could be significantly larger by adding additional AVL trees to the account. The program then would just look up any given mint address in the first tree and if it fails to find it, it would check the next, etc. Given the zero-copy data structure of the design, this would not entail deserializing and loading all the trees into memory so would have little to no compute cost to implement.

The specification currently does not support a delegate system as storing the extra data for that raises the costs significantly. However, delegates could likely be implemented in a cheaper and modular way but using an additional PDA to represent the delegation so that only use-cases that actually require delegates end up paying for them.

