use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::SharePoolError;

use super::{Key, SharePoolAccount, SharePoolAccountPdaArgs, SharePoolAccountSpaceArgs};

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct Pool {
    pub key: Key,
    pub collection: Pubkey,
    pub authority: Pubkey,
    pub shares_per_token: u64,
    pub pool_nfts: Vec<(Pubkey, Pubkey)>,
}

impl Pool {
    pub fn new(collection: Pubkey, authority: Pubkey, shares_per_token: u64) -> Self {
        Self {
            key: Key::Pool,
            collection,
            authority,
            shares_per_token,
            pool_nfts: Vec::new(),
        }
    }
}

impl SharePoolAccount for Pool {
    fn space(args: SharePoolAccountSpaceArgs) -> Result<usize, SharePoolError> {
        #[allow(irrefutable_let_patterns)]
        if let SharePoolAccountSpaceArgs::Pool { pool_nfts } = args {
            let mut space = 1; // Key
            space += 32; // collection
            space += 32; // authority
            space += 8; // shares_per_token
            space += 4 + (pool_nfts * (32 + 32)); // pool_nfts
            return Ok(space);
        }
        Err(SharePoolError::InvalidSpaceArgs)
    }

    fn seeds(args: SharePoolAccountPdaArgs) -> Result<Vec<&[u8]>, SharePoolError> {
        #[allow(irrefutable_let_patterns)]
        if let SharePoolAccountPdaArgs::Pool {
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
