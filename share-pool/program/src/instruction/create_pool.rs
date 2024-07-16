use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct CreatePoolArgs {
    pub shares_per_token: u64,
}
