use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum SharePoolError {
    /// 0 - Error deserializing an account
    #[error("Error deserializing an account")]
    DeserializationError,
    /// 1 - Error serializing an account
    #[error("Error serializing an account")]
    SerializationError,
    /// 2 - Invalid program owner
    #[error("Invalid program owner. This likely mean the provided account does not exist")]
    InvalidProgramOwner,
    /// 3 - Invalid PDA derivation
    #[error("Invalid PDA derivation")]
    InvalidPda,
    /// 4 - Expected empty account
    #[error("Expected empty account")]
    ExpectedEmptyAccount,
    /// 5 - Expected non empty account
    #[error("Expected non empty account")]
    ExpectedNonEmptyAccount,
    /// 6 - Expected signer account
    #[error("Expected signer account")]
    ExpectedSignerAccount,
    /// 7 - Expected writable account
    #[error("Expected writable account")]
    ExpectedWritableAccount,
    /// 8 - Account mismatch
    #[error("Account mismatch")]
    AccountMismatch,
    /// 9 - Invalid account key
    #[error("Invalid account key")]
    InvalidAccountKey,
    /// 10 - Numerical overflow
    #[error("Numerical overflow")]
    NumericalOverflow,
    /// 11 - Invalid space args
    #[error("Invalid space args")]
    InvalidSpaceArgs,
    /// 12 - Invalid seed args
    #[error("Invalid seed args")]
    InvalidSeedArgs,
    /// 13 - Invalid seeds
    #[error("Invalid seeds")]
    InvalidSeeds,
    /// 14 - Invalid mint key
    #[error("Invalid mint key")]
    InvalidMintKey,
}

impl PrintProgramError for SharePoolError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<SharePoolError> for ProgramError {
    fn from(e: SharePoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SharePoolError {
    fn type_of() -> &'static str {
        "Share Pool Error"
    }
}
