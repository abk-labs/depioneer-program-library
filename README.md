# SPL-302 Prototype

Program identifier: Vanity address starting with SPL3

# Instructions:

## `create_pool`

Creates a pool associated with an nft collection.

### args:

- `shares_per_token (u64)` - Number of shares that each token on the collection represents.

### accounts:

- `pool (writable)` - Pool of tokens.
    - Pda: [b”pool”, collection_nft.pubkey()]
- `collection_nft`  - Collection NFT to associate the pool to.
- `collection_authority (signer)`  - Collection update authority.
- `payer (writable, signer)`  - Transaction payer.
- `system_program` - System program for allocation.

## `update_pool`

Updates a pool associated with an nft collection.

### args:

- `shares_per_token (u64)` - Number of shares that each token on the collection represents.

### accounts:

- `pool (writable)` - Pool of tokens.
    - Pda: [b”pool”, collection_nft.pubkey()]
- `collection_nft`  - Collection NFT to associate the pool to.
- `collection_authority (signer)`  - Collection update authority.
- `payer (writable, signer)`  - Transaction payer.
- `system_program` - System program for allocation.

## `create_pool_token_account`

Creates a pool token account for the given mint. Afterwards, the pool can be read and be sent tokens to with regular transfers.

### args:

### accounts:

- `pool (writable)` - Pool of tokens.
- `mint`  - Mint to add.
- `pool_token_account (writable)` - New pool token account to save.
    - Pda: [b”pool_token_account”, pool.pubkey(), mint.pubkey()]
- `owner (signer)` - Owner of the source token account.
- `payer (writable, signer)`  - Transaction payer.
- `token_program`  - Token program for transfer.
- `system_program` - System program for allocation.

## `redeem`

Redeems one token from the collection for it’s equivalent in shares of the underlying tokens.

# Accounts:

- `pool (writable)` - Pool of tokens.
- `nft_metadata` - Nft to check.
- `nft_mint` - Nft mint.
- `nft_token_account (writable)`  - Nft token account.
- `pool_nft_token_account (writable)`  - Pool Nft token account.
- `owner` - Owner of the receiving accounts and the pool nft.
- `metadata_program` - Metadata program
- `token_program`  - Token program
- `system_program` - System program for allocation.
- … remaining `mint` / `token_account (writable)`  groups of pairs (3 accounts represent a transfer from contract to payer).
    - (sourceAta(contract): Account, destinationAta: Account, mint: Account)[]

## `remint`

Mints back an nft from the pool price

# Accounts:

- `pool (writable)` - Pool of tokens.
- `nft_metadata` - Nft to check.
- `nft_mint` - Nft mint.
- `nft_token_account (writable)`  - Nft token account.
- `pool_nft_token_account (writable)`  - Pool Nft token account.
- `metadata_program` - Metadata program
- `token_program`  - Token program
- `system_program` - System program for allocation.
- … remaining `mint` / `token_account (writable)`  pairs of pairs to be used as payment. (3 accounts represent a transfer from contract to payer).
    - (sourceAta(Client): Account, destinationAta(Contract): Account, mint: Account)[]

# Account structs

```rust
// Pda: [b”pool”, collection_nft.pubkey()]
pub struct Pool {
  collection: Pubkey;
  shares_per_token: u64; // 
  // Token accounts are mint + token account pairs.
  pool_token_accounts: Vec<(Pubkey, Pubkey)>;
  // Nfts that are being held by the pool.
  pool_nfts: Vec<(Pubkey, Pubkey)>;
}
```