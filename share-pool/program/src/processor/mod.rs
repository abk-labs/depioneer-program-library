pub mod create_pool;

use borsh::BorshDeserialize;
use create_pool::create_pool;
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
        }
    }
}
