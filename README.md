<h1 align="center">
  <code>Sigil</code>
</h1>
<p align="center">
  <img width="400" alt="Sigil" src="https://github.com/nifty-oss/sigil/assets/1684605/bc3643c3-8623-45f4-ab63-783b0bea2ce7" />
</p>
<p align="center">
  A cost-efficient standard for fungible assets.
</p>

<p align="center">
<a href="https://explorer.solana.com/address/BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9"><img src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnifty-oss%2Fsigil%2Fmain%2Fprogram%2Fidl.json&query=%24.version&label=program&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iMzEzIiBoZWlnaHQ9IjI4MSIgdmlld0JveD0iMCAwIDMxMyAyODEiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxnIGNsaXAtcGF0aD0idXJsKCNjbGlwMF80NzZfMjQzMCkiPgo8cGF0aCBkPSJNMzExLjMxOCAyMjEuMDU3TDI1OS42NiAyNzYuNTU4QzI1OC41MzcgMjc3Ljc2NCAyNTcuMTc4IDI3OC43MjUgMjU1LjY2OSAyNzkuMzgyQzI1NC4xNTkgMjgwLjAzOSAyNTIuNTMgMjgwLjM3OCAyNTAuODg0IDI4MC4zNzdINS45OTcxOUM0LjgyODcgMjgwLjM3NyAzLjY4NTY4IDI4MC4wMzUgMi43MDg1NSAyNzkuMzkzQzEuNzMxNDMgMjc4Ljc1MSAwLjk2Mjc3MSAyNzcuODM3IDAuNDk3MDIgMjc2Ljc2NEMwLjAzMTI2OTEgMjc1LjY5IC0wLjExMTI4NiAyNzQuNTA0IDAuMDg2ODcxMiAyNzMuMzVDMC4yODUwMjggMjcyLjE5NiAwLjgxNTI2NSAyNzEuMTI2IDEuNjEyNDMgMjcwLjI3TDUzLjMwOTkgMjE0Ljc2OUM1NC40Mjk5IDIxMy41NjYgNTUuNzg0MyAyMTIuNjA3IDU3LjI4OTMgMjExLjk1QzU4Ljc5NDMgMjExLjI5MyA2MC40MTc4IDIxMC45NTMgNjIuMDU5NSAyMTAuOTVIMzA2LjkzM0MzMDguMTAxIDIxMC45NSAzMDkuMjQ0IDIxMS4yOTIgMzEwLjIyMSAyMTEuOTM0QzMxMS4xOTkgMjEyLjU3NiAzMTEuOTY3IDIxMy40OSAzMTIuNDMzIDIxNC41NjRDMzEyLjg5OSAyMTUuNjM3IDMxMy4wNDEgMjE2LjgyNCAzMTIuODQzIDIxNy45NzdDMzEyLjY0NSAyMTkuMTMxIDMxMi4xMTUgMjIwLjIwMSAzMTEuMzE4IDIyMS4wNTdaTTI1OS42NiAxMDkuMjk0QzI1OC41MzcgMTA4LjA4OCAyNTcuMTc4IDEwNy4xMjcgMjU1LjY2OSAxMDYuNDdDMjU0LjE1OSAxMDUuODEzIDI1Mi41MyAxMDUuNDc0IDI1MC44ODQgMTA1LjQ3NUg1Ljk5NzE5QzQuODI4NyAxMDUuNDc1IDMuNjg1NjggMTA1LjgxNyAyLjcwODU1IDEwNi40NTlDMS43MzE0MyAxMDcuMTAxIDAuOTYyNzcxIDEwOC4wMTUgMC40OTcwMiAxMDkuMDg4QzAuMDMxMjY5MSAxMTAuMTYyIC0wLjExMTI4NiAxMTEuMzQ4IDAuMDg2ODcxMiAxMTIuNTAyQzAuMjg1MDI4IDExMy42NTYgMC44MTUyNjUgMTE0LjcyNiAxLjYxMjQzIDExNS41ODJMNTMuMzA5OSAxNzEuMDgzQzU0LjQyOTkgMTcyLjI4NiA1NS43ODQzIDE3My4yNDUgNTcuMjg5MyAxNzMuOTAyQzU4Ljc5NDMgMTc0LjU1OSA2MC40MTc4IDE3NC44OTkgNjIuMDU5NSAxNzQuOTAySDMwNi45MzNDMzA4LjEwMSAxNzQuOTAyIDMwOS4yNDQgMTc0LjU2IDMxMC4yMjEgMTczLjkxOEMzMTEuMTk5IDE3My4yNzYgMzExLjk2NyAxNzIuMzYyIDMxMi40MzMgMTcxLjI4OEMzMTIuODk5IDE3MC4yMTUgMzEzLjA0MSAxNjkuMDI4IDMxMi44NDMgMTY3Ljg3NUMzMTIuNjQ1IDE2Ni43MjEgMzEyLjExNSAxNjUuNjUxIDMxMS4zMTggMTY0Ljc5NUwyNTkuNjYgMTA5LjI5NFpNNS45OTcxOSA2OS40MjY3SDI1MC44ODRDMjUyLjUzIDY5LjQyNzUgMjU0LjE1OSA2OS4wODkgMjU1LjY2OSA2OC40MzJDMjU3LjE3OCA2Ny43NzUxIDI1OC41MzcgNjYuODEzOSAyNTkuNjYgNjUuNjA4MkwzMTEuMzE4IDEwLjEwNjlDMzEyLjExNSA5LjI1MTA3IDMxMi42NDUgOC4xODA1NiAzMTIuODQzIDcuMDI2OTVDMzEzLjA0MSA1Ljg3MzM0IDMxMi44OTkgNC42ODY4NiAzMTIuNDMzIDMuNjEzM0MzMTEuOTY3IDIuNTM5NzQgMzExLjE5OSAxLjYyNTg2IDMxMC4yMjEgMC45ODM5NDFDMzA5LjI0NCAwLjM0MjAyNiAzMDguMTAxIDMuOTUzMTRlLTA1IDMwNi45MzMgMEw2Mi4wNTk1IDBDNjAuNDE3OCAwLjAwMjc5ODY2IDU4Ljc5NDMgMC4zNDMxNCA1Ny4yODkzIDAuOTk5OTUzQzU1Ljc4NDMgMS42NTY3NyA1NC40Mjk5IDIuNjE2MDcgNTMuMzA5OSAzLjgxODQ3TDEuNjI1NzYgNTkuMzE5N0MwLjgyOTM2MSA2MC4xNzQ4IDAuMjk5MzU5IDYxLjI0NCAwLjEwMDc1MiA2Mi4zOTY0Qy0wLjA5Nzg1MzkgNjMuNTQ4OCAwLjA0MzU2OTggNjQuNzM0MiAwLjUwNzY3OSA2NS44MDczQzAuOTcxNzg5IDY2Ljg4MDMgMS43Mzg0MSA2Ny43OTQzIDIuNzEzNTIgNjguNDM3MkMzLjY4ODYzIDY5LjA4MDIgNC44Mjk4NCA2OS40MjQgNS45OTcxOSA2OS40MjY3WiIgZmlsbD0idXJsKCNwYWludDBfbGluZWFyXzQ3Nl8yNDMwKSIvPgo8L2c+CjxkZWZzPgo8bGluZWFyR3JhZGllbnQgaWQ9InBhaW50MF9saW5lYXJfNDc2XzI0MzAiIHgxPSIyNi40MTUiIHkxPSIyODcuMDU5IiB4Mj0iMjgzLjczNSIgeTI9Ii0yLjQ5NTc0IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+CjxzdG9wIG9mZnNldD0iMC4wOCIgc3RvcC1jb2xvcj0iIzk5NDVGRiIvPgo8c3RvcCBvZmZzZXQ9IjAuMyIgc3RvcC1jb2xvcj0iIzg3NTJGMyIvPgo8c3RvcCBvZmZzZXQ9IjAuNSIgc3RvcC1jb2xvcj0iIzU0OTdENSIvPgo8c3RvcCBvZmZzZXQ9IjAuNiIgc3RvcC1jb2xvcj0iIzQzQjRDQSIvPgo8c3RvcCBvZmZzZXQ9IjAuNzIiIHN0b3AtY29sb3I9IiMyOEUwQjkiLz4KPHN0b3Agb2Zmc2V0PSIwLjk3IiBzdG9wLWNvbG9yPSIjMTlGQjlCIi8+CjwvbGluZWFyR3JhZGllbnQ+CjxjbGlwUGF0aCBpZD0iY2xpcDBfNDc2XzI0MzAiPgo8cmVjdCB3aWR0aD0iMzEyLjkzIiBoZWlnaHQ9IjI4MC4zNzciIGZpbGw9IndoaXRlIi8+CjwvY2xpcFBhdGg+CjwvZGVmcz4KPC9zdmc+Cg==&color=9945FF" /></a>
<a href="https://www.npmjs.com/package/@nifty-oss/sigil"><img src="https://img.shields.io/npm/v/%40nifty-oss%2Fsigil?logo=npm&color=377CC0" /></a>
<a href="https://crates.io/crates/sigil-client"><img src="https://img.shields.io/crates/v/sigil-client?logo=rust" /></a>
</p>


## Overview

Sigil is a novel fungible item standard and program on Solana that represents fungible items on-chain using minimal data, ensuring the lowest possible data storage costs. While off-chain data solutions, such as Merkle proofs, could be even cheaper, Sigil's on-chain approach offers the benefits of small transactions without requiring cumbersome proofs. Additionally, token data is directly accessible by other Solana programs via account state. Sigil strikes the optimal balance between on-chain, accessible data and minimal costs, considering the current limitations of account data on Solana's runtime.

The Sigil specification is not intended to replace existing token programs on Solana, which are well-established and feature-rich. Instead, it aims to capture new use cases for which the current standards are prohibitively expensive. For example, in gaming, a game studio may want to create numerous assets for users while subsidizing their rent costs to reduce friction. However, the current costs for creating new token accounts for each asset could become excessively high, given a large user base and multiple assets per user.

To ensure optimal efficiency in terms of compute and memory usage, the Sigil program is implemented with all data structs using zero-copy bytemuck implementations.

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

In the SPL token program the mint account is 82 bytes in size, plus the standard account info overhead of 128 bytes, but only has to be created once per asset, so typically represents a fixed up-front cost that is paid initially but does not scale up by number of users. Sigil's mint account is not much smaller, but does save a few bytes coming in at 58 bytes.

Token accounts however have significant savings, as SPL token accounts require a new token account per user and mint which is 128 bytes + 165 bytes for a total of 293 bytes. In Sigil, there is a fixed cost of 128 bytes + 68 bytes for a new user token account, but then each additional asset only requires 12 bytes without having to pay for the 128 account info header each time as the pairs are simply stored in the AVL tree on the same account.

**Base Comparisons**

| Account              | Data Size (Bytes) | Rent Cost @ $200 SOL |
| -------------------- | ----------------- | -------------------- |
| SPL Mint             | 82                | $0.29                |
| TL Mint              | 58                | $0.26                |
| SPL Token Account    | 165               | $0.41                |
| Sigil Token Account  | 80                | $0.29                |



**User w/ 100 Assets**

| Account              | Data Size (Bytes) | Rent Cost @ $200 SOL |
| -------------------- | ----------------- | -------------------- |
| SPL Token Account    | 16,500            | $41                  |
| Sigil Token Account  | 1268              | $1.94                |



**1000 Users w/ 100 Assets**

| Account              | Data Size (Bytes) | Rent Cost @ $200 SOL |
| -------------------- | ----------------- | -------------------- |
| SPL Token Account    | 16,500,000        | $41,000              |
| Sigil Token Account  | 1,268,000         | $1,940               |



### Limitations

To save size, the AVL tree pointers are stored as u8s which means that each AVL tree can only store 255 mint/supply pairs. This is expected to be sufficient for most use-cases as users typically do not have more than a few hundred game assets or fungible tokens per wallet. However, given Solana accounts can be up to 10 MB in size, the amount stored could be significantly larger by adding additional AVL trees to the account. The program then would just look up any given mint address in the first tree and if it fails to find it, it would check the next, etc. Given the zero-copy data structure of the design, this would not entail deserializing and loading all the trees into memory so would have little to no compute cost to implement.

The specification currently does not support a delegate system as storing the extra data for that raises the costs significantly. However, delegates could likely be implemented in a cheaper and modular way but using an additional PDA to represent the delegation so that only use-cases that actually require delegates end up paying for them.


## Project setup for devs

To get started run the following command


```sh
pnpm install
```

to install the necessary dependencies to set up the project and run `pnpm` scripts.

Now you can build the program:


```sh
pnpm programs:build
```

generate the clients and IDL:

```
pnpm generate
```

start a local validator

```sh
pnpm validator:start
```

and run tests:

```sh
pnpm clients:js:test
pnpm clients:rust:test
```

## Managing clients

The following clients are available for the Sigil. You may use the following links to learn more about each client.

- [JS client](./clients/js)
- [Rust client](./clients/rust)

## Starting and stopping the local validator

The following script is available to start a local validator for testing.

```sh
pnpm validator:start
```

By default, if a local validator is already running, the script will be skipped. You may use the `validator:restart` script instead to force the validator to restart.

```sh
pnpm validator:restart
```

Finally, you may stop the local validator using the following command.

```sh
pnpm validator:stop
```
