# Sigil

A simple Token Program that minimizes on-chain state bond costs to store token accounts.

## Namespace

The program uses a namespace model to discourage "squatting" high-value tickers such as USDC. Without a namespace there would be a single "USDC" ticker globally for the whole program and this would incentivizing minting out common tickers. Putting tickers behind a namespace and requiring the namespace to sign the transaction to create token and mint accounts discourages squatting and allows people to be able to trust that a particular namespace + ticker combination is the particular one they are looking for.

Once a mint is created in a namespace, and a token account is created for a particular user, then it is permissionless to add additional (pre-existing mints) to that token account.

Creating a token account requires the namespace to sign the transaction to ensure that user's token accounts can only be created by the authority for that namespace. In addition, the namespace authority is required to sign the transaction to create a new Mint account in that name space.
