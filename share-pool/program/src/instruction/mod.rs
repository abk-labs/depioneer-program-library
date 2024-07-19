pub mod create_pool;

use borsh::{BorshDeserialize, BorshSerialize};
use create_pool::CreatePoolArgs;
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum CounterInstruction {
    #[account(0, writable, name="pool", desc="Pool account to create (seeds: ['pool', collection_nft, authority])")]
    #[account(1, name="collection_nft_mint", desc="Collection NFT Mint account")]
    #[account(2, name="collection_nft_metadata", desc="Collection NFT Metadata account")]
    #[account(3, signer, name="authority", desc="Authority account")]
    #[account(4, signer, writable, name="payer", desc="Payer account")]
    #[account(5, name="system_program", desc="System program account")]
    CreatePool { args: CreatePoolArgs },

    #[account(0, writable, name="pool", desc="Pool account to add mint (seeds: ['pool', mint, pool_token_account, authority])")]
    #[account(1, name="mint", desc="Mint account to add")]
    #[account(2, writable, name="pool_token_account", desc="Pool token account to save (seeds: ['pool_token_account', pool, mint])")]
    #[account(3, signer, name="authority", desc="Authority account")]
    #[account(4, signer, writable, name="payer", desc="Payer account")]
    #[account(5, name="token_program", desc="Token program account")]
    #[account(6, name="system_program", desc="System program account")]
    #[account(7, name="rent", desc="Rent sysvar account")]
    CreatePoolTokenAccount,
}
