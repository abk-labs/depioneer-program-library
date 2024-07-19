use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::SharePoolError;

use super::{
    AtaMintPair, Key, SharePoolAccount, SharePoolAccountSeedsArgs, SharePoolAccountSpaceArgs,
};

#[repr(C)]
#[derive(ShankAccount, BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct Pool {
    pub key: Key,
    pub bump: u8,
    pub collection: Pubkey,
    pub authority: Pubkey,
    pub shares_per_token: u64,
    pub pool_nfts: Vec<AtaMintPair>,
    pub pool_token_accounts: Vec<AtaMintPair>,
}

impl Pool {
    pub fn new(bump: u8, collection: Pubkey, authority: Pubkey, shares_per_token: u64) -> Self {
        Self {
            key: Key::Pool,
            bump,
            collection,
            authority,
            shares_per_token,
            pool_nfts: Vec::new(),
            pool_token_accounts: Vec::new(),
        }
    }
}

impl SharePoolAccount for Pool {
    fn space(args: SharePoolAccountSpaceArgs) -> Result<usize, SharePoolError> {
        #[allow(irrefutable_let_patterns)]
        if let SharePoolAccountSpaceArgs::Pool {
            pool_nfts,
            pool_token_accounts,
        } = args
        {
            let mut space = 1; // Key
            space += 1; // bump
            space += 32; // collection
            space += 32; // authority
            space += 8; // shares_per_token
            space += 4 + (pool_nfts * AtaMintPair::LEN); // pool_nfts
            space += 4 + (pool_token_accounts * AtaMintPair::LEN); // pool_token_accounts
            return Ok(space);
        }
        Err(SharePoolError::InvalidSpaceArgs)
    }

    fn seeds(args: SharePoolAccountSeedsArgs) -> Result<Vec<&[u8]>, SharePoolError> {
        #[allow(irrefutable_let_patterns)]
        if let SharePoolAccountSeedsArgs::Pool {
            collection,
            authority,
        } = args
        {
            return Ok(vec![
                "pool".as_bytes(),
                collection.as_ref(),
                authority.as_ref(),
            ]);
        }
        Err(SharePoolError::InvalidSeedArgs)
    }
}
