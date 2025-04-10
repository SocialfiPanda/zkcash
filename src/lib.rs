use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// Modules
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
pub mod verifier;
pub mod poseidon;

// Entrypoint
entrypoint!(process_instruction);

// Process instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::Processor::process(program_id, accounts, instruction_data)
}

// Merkle tree verification
pub fn verify_merkle_proof(
    leaf: &[u8; 32],
    path: &[[u8; 32]],
    indices: &[u8],
    root: &[u8; 32],
) -> Result<bool, solana_program::program_error::ProgramError> {
    let calculated_root = poseidon::compute_merkle_root(leaf, path, indices)?;
    Ok(calculated_root == *root)
}
