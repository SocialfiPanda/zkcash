use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized},
    program_error::ProgramError,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Pool {
    pub is_initialized: bool,
    pub merkle_tree_height: u8,
    pub total_amount: u64,
}

impl IsInitialized for Pool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MerkleTree {
    pub is_initialized: bool,
    pub height: u8,
    pub current_index: u32,
    pub root: [u8; 32],
    pub filled_subtrees: Vec<[u8; 32]>,
}

impl MerkleTree {
    pub fn new(height: u8) -> Self {
        let mut filled_subtrees = Vec::with_capacity(height as usize);
        let zero_value = [0u8; 32];
        
        for _ in 0..height {
            filled_subtrees.push(zero_value);
        }
        
        Self {
            is_initialized: true,
            height,
            current_index: 0,
            root: zero_value,
            filled_subtrees,
        }
    }
    
    pub fn insert(&mut self, leaf: &[u8; 32]) -> Result<(), ProgramError> {
        if self.current_index as usize >= (1 << self.height) {
            return Err(ProgramError::InvalidArgument);
        }
        
        let mut current_index = self.current_index;
        let current = *leaf;
        
        for i in 0..self.height as usize {
            if current_index % 2 == 0 {
                // Current is left, filled_subtree is right
                self.filled_subtrees[i] = current;
                self.root = crate::poseidon::hash_left_right(&current, &self.filled_subtrees[i])?;
            } else {
                // Current is right, filled_subtree is left
                self.root = crate::poseidon::hash_left_right(&self.filled_subtrees[i], &current)?;
            }
            
            // Move up one level in the tree
            current_index /= 2;
        }
        
        self.current_index += 1;
        Ok(())
    }
}

impl IsInitialized for MerkleTree {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Nullifier {
    pub is_initialized: bool,
    pub nullifier_hash: [u8; 32],
}

impl IsInitialized for Nullifier {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
