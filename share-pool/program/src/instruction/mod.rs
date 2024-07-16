pub mod create_pool;

use borsh::{BorshDeserialize, BorshSerialize};
use create_pool::CreatePoolArgs;
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum CounterInstruction {
    #[account(0, writable, name="pool", desc="Pool account to create (seeds: ['pool', collection_nft])")]
    #[account(1, name="collection_nft", desc="Collection NFT Metadata account")]
    #[account(2, signer, name="authority", desc="Authority account")]
    #[account(3, signer, writable, name="payer", desc="Payer account")]
    #[account(4, name="system_program", desc="System program account")]
    CreatePool { args: CreatePoolArgs },
}
