use solana_program::{
    program_error::ProgramError,
};

use light_poseidon::{Poseidon, PoseidonBytesHasher};
use ark_bn254::Fr;
use once_cell::sync::Lazy;

// Implementation using light-poseidon library for Poseidon hash

// Create a thread local Poseidon instance for each width
static POSEIDON_INIT: Lazy<()> = Lazy::new(|| {
    // Just to ensure the lazy static is initialized
});

// Create new Poseidon instance for width 1
fn get_poseidon_width_1() -> Result<Poseidon<Fr>, ProgramError> {
    // Ensure initialization is performed
    Lazy::force(&POSEIDON_INIT);
    
    // Create a new instance each time (they're lightweight)
    Poseidon::<Fr>::new_circom(1)
        .map_err(|_| ProgramError::Custom(1))
}

// Create new Poseidon instance for width 2
fn get_poseidon_width_2() -> Result<Poseidon<Fr>, ProgramError> {
    // Ensure initialization is performed
    Lazy::force(&POSEIDON_INIT);
    
    // Create a new instance each time (they're lightweight)
    Poseidon::<Fr>::new_circom(2)
        .map_err(|_| ProgramError::Custom(1))
}

// Simple wrapper for hashing a single input
pub fn hash_1(input: &[u8; 32]) -> Result<[u8; 32], ProgramError> {
    let mut poseidon = get_poseidon_width_1()?;
    
    poseidon.hash_bytes_be(&[input])
        .map_err(|_| ProgramError::Custom(1))
}

// Implementation for hashing two inputs
pub fn hash_2(left: &[u8; 32], right: &[u8; 32]) -> Result<[u8; 32], ProgramError> {
    let mut poseidon = get_poseidon_width_2()?;
    
    poseidon.hash_bytes_be(&[left, right])
        .map_err(|_| ProgramError::Custom(1))
}

// Alias for hash_2 to maintain API compatibility
pub fn hash_left_right(left: &[u8; 32], right: &[u8; 32]) -> Result<[u8; 32], ProgramError> {
    hash_2(left, right)
}

// Compute Merkle root from leaf and path
pub fn compute_merkle_root(
    leaf: &[u8; 32],
    path: &[[u8; 32]],
    indices: &[u8],
) -> Result<[u8; 32], ProgramError> {
    // Check for empty path
    if path.is_empty() {
        return Err(PoseidonError::InvalidInputLength.into());
    }
    
    // Check indices length
    if indices.len() * 8 < path.len() {
        return Err(PoseidonError::InvalidInputLength.into());
    }
    
    let mut current = *leaf;
    
    for i in 0..path.len() {
        let path_element = path[i];
        let index_bit = indices[i / 8] & (1 << (i % 8));
        
        if index_bit == 0 {
            // Current is left, path_element is right
            current = hash_left_right(&current, &path_element)?;
        } else {
            // Current is right, path_element is left
            current = hash_left_right(&path_element, &current)?;
        }
    }
    
    Ok(current)
}

// Error type for Poseidon operations
#[derive(Debug)]
pub enum PoseidonError {
    InvalidInputLength,
    VecToArray,
    PoseidonHashError,
}

impl From<PoseidonError> for ProgramError {
    fn from(_e: PoseidonError) -> Self {
        ProgramError::Custom(1) // Use a custom error code
    }
}
