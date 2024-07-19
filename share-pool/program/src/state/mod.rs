pub mod ata_mint_pair;
pub mod pool;

pub use ata_mint_pair::*;
pub use pool::*;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::SharePoolError;

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum Key {
    Uninitialized,
    Pool,
    Redemption,
}

#[derive(Clone, Debug)]
pub enum SharePoolAccountSpaceArgs {
    /// Args needed for the pool account.
    Pool {
        pool_nfts: usize,
        pool_token_accounts: usize,
    },
    Redemption {
        items: usize,
    },
}

pub enum SharePoolAccountSeedsArgs<'a> {
    /// Seeds needed for the pool account.
    Pool {
        collection: &'a Pubkey,
        authority: &'a Pubkey,
    },
    Redemption {
        pool: &'a Pubkey,
        authority: &'a Pubkey,
    },
}

/// Common interface for all this program accounts, allowing for easy loading/saving,
/// seed calculation, and space calculation.
pub trait SharePoolAccount: BorshSerialize + BorshDeserialize {
    /// Loads the account from the given account info.
    fn load(account: &AccountInfo) -> Result<Self, ProgramError> {
        let mut bytes: &[u8] = &(*account.data).borrow();
        Self::deserialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            SharePoolError::DeserializationError.into()
        })
    }

    /// Writes the account to the given account info.
    fn save(&self, account: &AccountInfo) -> ProgramResult {
        borsh::to_writer(&mut account.data.borrow_mut()[..], self).map_err(|error| {
            msg!("Error: {}", error);
            SharePoolError::SerializationError.into()
        })
    }

    /// Returns the seeds for the account, without the program id or bump seed.
    fn seeds(args: SharePoolAccountSeedsArgs) -> Result<Vec<&[u8]>, SharePoolError>;

    /// Returns the PDA and bump seed for the account.
    fn pda(args: SharePoolAccountSeedsArgs) -> Result<(Pubkey, u8), SharePoolError> {
        Ok(Pubkey::find_program_address(
            &Self::seeds(args)?,
            &crate::ID,
        ))
    }

    /// Calculates the space of the account.
    fn space(args: SharePoolAccountSpaceArgs) -> Result<usize, SharePoolError>;
}
