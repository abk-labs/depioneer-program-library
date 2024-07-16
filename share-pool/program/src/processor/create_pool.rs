use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, system_program};

use crate::{
    assertions::{
        assert_empty, assert_pda, assert_program_owner, assert_same_pubkeys, assert_signer,
        assert_writable,
    },
    instruction::{accounts::CreatePoolAccounts, create_pool::CreatePoolArgs},
    state::{Pool, SharePoolAccount, SharePoolAccountPdaArgs, SharePoolAccountSpaceArgs},
    utils::create_account,
};

pub fn create_pool<'a>(accounts: &'a [AccountInfo<'a>], args: CreatePoolArgs) -> ProgramResult {
    // Accounts.
    let ctx = CreatePoolAccounts::context(accounts)?;

    // Guards.
    let pool_seeds = Pool::seeds(SharePoolAccountPdaArgs::Pool {
        collection: ctx.accounts.pool.key,
        authority: ctx.accounts.authority.key,
    })?;
    let pool_bump = assert_pda("pool", ctx.accounts.pool, &crate::ID, &pool_seeds)?;
    
    assert_empty("pool", ctx.accounts.pool)?;
    assert_program_owner(
        "collection_nft",
        ctx.accounts.collection_nft,
        &mpl_token_metadata::ID,
    )?;
    assert_signer("authority", ctx.accounts.authority)?;
    assert_signer("payer", ctx.accounts.payer)?;
    assert_writable("payer", ctx.accounts.payer)?;
    assert_same_pubkeys(
        "system_program",
        ctx.accounts.system_program,
        &system_program::ID,
    )?;

    let mut pool_seeds = pool_seeds;
    let pool_bump_seed = [pool_bump];
    pool_seeds.push(&pool_bump_seed);

    create_account(
        ctx.accounts.pool,
        ctx.accounts.payer,
        ctx.accounts.system_program,
        Pool::space(SharePoolAccountSpaceArgs::Pool { pool_nfts: 0 })?,
        &crate::ID,
        Some(&[&pool_seeds]),
    )?;

    Pool::new(
        *ctx.accounts.pool.key,
        *ctx.accounts.authority.key,
        args.shares_per_token,
    )
    .save(ctx.accounts.pool)?;

    Ok(())
}
