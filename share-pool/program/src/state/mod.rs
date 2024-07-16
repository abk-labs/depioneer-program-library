pub mod pool;

use borsh::{BorshDeserialize, BorshSerialize};
pub use pool::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::SharePoolError;

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum Key {
    Uninitialized,
    Pool,
}

#[derive(Clone, Debug)]
pub enum SharePoolAccountSpaceArgs {
    Pool { pool_nfts: usize },
}

pub enum SharePoolAccountPdaArgs<'a> {
    Pool {
        collection: &'a Pubkey,
        authority: &'a Pubkey,
    },
}

pub trait SharePoolAccount: BorshSerialize + BorshDeserialize {
    fn load(account: &AccountInfo) -> Result<Self, ProgramError> {
        let mut bytes: &[u8] = &(*account.data).borrow();
        Self::deserialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            SharePoolError::DeserializationError.into()
        })
    }
    fn save(&self, account: &AccountInfo) -> ProgramResult {
        borsh::to_writer(&mut account.data.borrow_mut()[..], self).map_err(|error| {
            msg!("Error: {}", error);
            SharePoolError::SerializationError.into()
        })
    }
    fn seeds(args: SharePoolAccountPdaArgs) -> Result<Vec<&[u8]>, SharePoolError>;
    fn find_pda(args: SharePoolAccountPdaArgs) -> Result<(Pubkey, u8), SharePoolError> {
        let seeds = &Self::seeds(args)?;
        return Ok(Pubkey::find_program_address(seeds, &crate::ID));
    }
    fn space(args: SharePoolAccountSpaceArgs) -> Result<usize, SharePoolError>;
}
