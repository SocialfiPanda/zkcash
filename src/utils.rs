use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Utils;

impl Utils {
    pub fn derive_commitment(secret: &[u8; 32], nullifier: &[u8; 32]) -> Result<[u8; 32], ProgramError> {
        crate::poseidon::hash_2(secret, nullifier)
    }
    
    pub fn derive_nullifier_hash(secret: &[u8; 32]) -> Result<[u8; 32], ProgramError> {
        crate::poseidon::hash_1(secret)
    }
    
    pub fn bytes_to_pubkey(bytes: &[u8; 32]) -> Pubkey {
        Pubkey::new_from_array(*bytes)
    }
}

pub fn find_pool_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"privacy_pool"], program_id)
}

pub fn find_merkle_tree_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"merkle_tree"], program_id)
}

pub fn find_nullifier_pda(program_id: &Pubkey, nullifier: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"nullifier", nullifier], program_id)
}

pub fn compute_merkle_path(
    _index: u32,
    tree_height: u8,
    filled_subtrees: &[[u8; 32]],
) -> Result<Vec<[u8; 32]>, ProgramError> {
    let mut path = Vec::with_capacity(tree_height as usize);
    
    for i in 0..tree_height as usize {
        let sibling = filled_subtrees[i];
        path.push(sibling);
    }
    
    Ok(path)
}

pub fn verify_merkle_proof(
    leaf: &[u8; 32],
    path: &[[u8; 32]],
    index: u32,
    root: &[u8; 32],
) -> Result<bool, ProgramError> {
    let mut current = *leaf;
    let mut current_index = index;
    
    for i in 0..path.len() {
        let sibling = &path[i];
        
        if current_index % 2 == 0 {
            current = crate::poseidon::hash_left_right(&current, sibling)?;
        } else {
            current = crate::poseidon::hash_left_right(sibling, &current)?;
        }
        
        current_index /= 2;
    }
    
    Ok(current == *root)
}
