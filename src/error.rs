use solana_program::{
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum PrivacyError {
    InvalidPool,
    InvalidRoot,
    InvalidProof,
    NullifierAlreadyUsed,
    InvalidRecipient,
    InsufficientFunds,
}

impl From<PrivacyError> for ProgramError {
    fn from(_e: PrivacyError) -> Self {
        ProgramError::Custom(1) // Use a custom error code
    }
}
