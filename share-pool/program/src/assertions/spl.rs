use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
use spl_token_2022::{extension::StateWithExtensions, state::Account};

use crate::error::SharePoolError;

use super::assert_program_owners;

/// Assert that the given account is owned by the token program or the token program 2022.
pub fn assert_owned_by_token_program_interface(
    account_name: &str,
    account: &AccountInfo,
) -> ProgramResult {
    assert_program_owners(account_name, account, &[spl_token::ID, spl_token_2022::ID])
}

/// Assert that the given token account has the expected mint pubkey.
pub fn assert_token_account_mint_pubkey(
    account_name: &str,
    token_account: &AccountInfo,
    deserialized_token_account: &StateWithExtensions<Account>,
    mint_account_pubkey: &Pubkey,
) -> ProgramResult {
    if deserialized_token_account.base.mint != *mint_account_pubkey {
        msg!(
            "Account \"{}\" [{}] expected mint [{}], got [{}]",
            account_name,
            token_account.key,
            mint_account_pubkey,
            deserialized_token_account.base.mint,
        );
        return Err(SharePoolError::InvalidMintKey.into());
    }
    Ok(())
}
