pub mod create_pool;
pub mod create_pool_token_account;

pub use create_pool::create_pool;
pub use create_pool_token_account::create_pool_token_account;

use borsh::BorshDeserialize;

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::CounterInstruction;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: CounterInstruction = CounterInstruction::try_from_slice(instruction_data)?;
    match instruction {
        CounterInstruction::CreatePool { args } => {
            msg!("Instruction: Create Pool");
            create_pool(accounts, args)
        },
        CounterInstruction::CreatePoolTokenAccount => {
            msg!("Instruction: Create Pool Token Account");
            create_pool_token_account(accounts)
        },
    }
}
