use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum DepositError {
    #[error("Invalid settings account")]
    InvalidSettingsAccount,
    #[error("Invalid data account")]
    InvalidDataAccount,
    #[error("Already initialized")]
    AlreadyInit,
    #[error("Not owner")]
    NotOwner,
    #[error("Not enough lamports")]
    NotEnough
}

impl From<DepositError> for ProgramError {
    fn from(e: DepositError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

