use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct AtaMintPair {
    pub ata: Pubkey,
    pub mint: Pubkey,
}

impl AtaMintPair {
    pub const LEN: usize = 32 + 32;
    pub fn new(ata: Pubkey, mint: Pubkey) -> Self {
        Self { ata, mint }
    }
}
