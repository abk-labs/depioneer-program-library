// #![cfg(feature = "test-sbf")]

use std::str::FromStr;

use borsh::BorshDeserialize;
use depioneer_share_pool_client::{accounts::Pool, instructions::CreatePoolBuilder};
use solana_program_test::{tokio, ProgramTest, ProgramTestContext};
use solana_sdk::{
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

async fn create_mint(ctx: &mut ProgramTestContext) -> Result<Keypair, ProgramError> {
    let mint = Keypair::new();
    let len = spl_token::state::Mint::LEN;
    let rent = ctx.banks_client.get_rent().await.unwrap();
    let minimum_balance = rent.minimum_balance(len);
    let create_account_ix = system_instruction::create_account(
        &ctx.payer.pubkey(),
        &mint.pubkey(),
        minimum_balance,
        len as u64,
        &spl_token::ID,
    );
    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &ctx.payer.pubkey(),
        None,
        0,
    )?;
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer, &mint],
        ctx.last_blockhash,
    );
    ctx.banks_client.process_transaction(tx).await.unwrap();
    Ok(mint)
}

// Tests the create_pool instruction
// Preconditions:
// 1. The program test is started with the depioneer_share_pool program
// 2. The payer has enough lamports to create the pool account
// 3. There is a mint account to use as the collection NFT
#[tokio::test]
async fn create_pool() {
    let mut program_test = ProgramTest::new(
        "depioneer_share_pool",
        depioneer_share_pool_client::ID,
        None,
    );
    program_test.add_builtin_program(
        "metaplex_token_metadata_program",
        Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
        |_, _, _, _, _, _| {},
    );
    let mut context = program_test.start_with_context().await;

    let collection_nft_mint = create_mint(&mut context).await.unwrap();

    // TODO: FIX PDA GENERATION IN CLIENT (@kespinola maybe?)
    let (pool_address, _pool_bump) = Pubkey::find_program_address(
        &[
            "pool".as_bytes(),
            collection_nft_mint.pubkey().as_ref(),
            context.payer.pubkey().as_ref(),
        ],
        &depioneer_share_pool_client::ID,
    );

    let ix = CreatePoolBuilder::new()
        .authority(context.payer.pubkey())
        .payer(context.payer.pubkey())
        .collection_nft(collection_nft_mint.pubkey())
        .pool(pool_address)
        .shares_per_token(1)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
    let account = context
        .banks_client
        .get_account(pool_address)
        .await
        .unwrap();
    assert!(account.is_some());

    let account = account.unwrap();
    assert!(account.data.len() > 0);

    let mut account_data = account.data.as_ref();
    let pool = Pool::deserialize(&mut account_data).unwrap();
    assert_eq!(pool.authority, context.payer.pubkey());
}
