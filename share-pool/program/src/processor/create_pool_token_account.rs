use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_pack::Pack,
};
use spl_token_2022::instruction::initialize_account;

use crate::{
    assertions::{
        assert_empty, assert_non_empty, assert_owned_by_token_program_interface,
        assert_pda_with_bump, assert_program_owner, assert_same_pubkeys, assert_signer,
        assert_writable,
    },
    instruction::accounts::CreatePoolTokenAccountAccounts,
    state::{Pool, SharePoolAccount, SharePoolAccountSeedsArgs, SharePoolAccountSpaceArgs},
    utils::{create_account, realloc_account},
};

pub fn create_pool_token_account<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreatePoolTokenAccountAccounts::context(accounts)?;

    // Guards.
    let pool_seeds = Pool::seeds(SharePoolAccountSeedsArgs::Pool {
        collection: ctx.accounts.pool.key,
        authority: ctx.accounts.authority.key,
    })?;
    let mut pool = Pool::load(ctx.accounts.pool)?;
    let mut pool_seeds_with_bump = pool_seeds.clone();
    let pool_bump_seed = [pool.bump];
    pool_seeds_with_bump.push(&pool_bump_seed);

    assert_pda_with_bump("pool", ctx.accounts.pool, &crate::ID, &pool_seeds_with_bump)?;
    assert_non_empty("pool", ctx.accounts.pool)?;
    assert_program_owner("pool", ctx.accounts.pool, &crate::ID)?;
    assert_writable("pool", ctx.accounts.pool)?;

    assert_non_empty("mint", ctx.accounts.mint)?;
    assert_owned_by_token_program_interface("mint", ctx.accounts.mint)?;

    assert_writable("pool_token_account", ctx.accounts.pool_token_account)?;
    assert_empty("pool_token_account", ctx.accounts.pool_token_account)?;

    assert_signer("authority", ctx.accounts.authority)?;

    assert_signer("payer", ctx.accounts.payer)?;
    assert_writable("payer", ctx.accounts.payer)?;

    assert_same_pubkeys("authority", ctx.accounts.authority, &pool.authority)?;

    // System-create token account.
    create_account(
        ctx.accounts.pool_token_account,
        ctx.accounts.payer,
        ctx.accounts.system_program,
        spl_token_2022::state::Account::LEN,
        ctx.accounts.mint.owner, // Already did the assertion above.
        None,
    )?;

    // Initialize token account.
    invoke(
        &initialize_account(
            ctx.accounts.token_program.key,
            ctx.accounts.pool_token_account.key,
            ctx.accounts.mint.key,
            ctx.accounts.pool.key,
        )?,
        &[
            ctx.accounts.pool_token_account.clone(),
            ctx.accounts.mint.clone(),
            ctx.accounts.pool.clone(),
            ctx.accounts.rent.clone(),
        ],
    )?;

    let token_account_mint_pair = (*ctx.accounts.pool_token_account.key, *ctx.accounts.mint.key);
    pool.pool_token_accounts.push(token_account_mint_pair);
    pool.save(ctx.accounts.pool)?;

    let new_pool_space = Pool::space(SharePoolAccountSpaceArgs::Pool {
        pool_nfts: pool.pool_nfts.len(),
        pool_token_accounts: pool.pool_token_accounts.len(),
    })?;

    realloc_account(
        ctx.accounts.pool,
        ctx.accounts.payer,
        ctx.accounts.system_program,
        new_pool_space,
        true,
    )?;

    Ok(())
}
