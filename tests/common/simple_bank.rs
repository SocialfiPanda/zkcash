use std::collections::HashMap;
use solana_program::pubkey::Pubkey;
use borsh::{BorshSerialize, BorshDeserialize};
use zkcash::state::{Pool, MerkleTree, Nullifier};
use zkcash::utils::{find_pool_pda, find_merkle_tree_pda, find_nullifier_pda};
use zkcash::error::PrivacyError;

/// A simple mock bank for testing the ZKCash protocol
pub struct SimpleBank {
    /// Map of all accounts in the bank
    accounts: HashMap<Pubkey, Vec<u8>>,
    /// Program ID for the ZKCash program
    program_id: Pubkey,
}

impl SimpleBank {
    /// Create a new SimpleBank with the given program ID
    pub fn new(program_id: Pubkey) -> Self {
        Self {
            accounts: HashMap::new(),
            program_id,
        }
    }

    /// Initialize the ZKCash protocol with a pool and merkle tree
    pub fn initialize(&mut self, merkle_tree_height: u8) -> Result<(), PrivacyError> {
        // Create the pool and merkle tree PDAs
        let (pool_pda, _) = find_pool_pda(&self.program_id);
        let (merkle_tree_pda, _) = find_merkle_tree_pda(&self.program_id);
        
        // Check if the pool is already initialized
        if self.accounts.contains_key(&pool_pda) {
            return Err(PrivacyError::InvalidPool);
        }
        
        // Create the pool
        let pool = Pool {
            is_initialized: true,
            merkle_tree_height,
            total_amount: 0,
        };
        
        // Manually serialize the pool
        let mut pool_data = vec![];
        pool_data.push(pool.is_initialized as u8);
        pool_data.push(pool.merkle_tree_height);
        pool_data.extend_from_slice(&pool.total_amount.to_le_bytes());
        
        self.accounts.insert(pool_pda, pool_data);
        
        // Create the merkle tree
        let merkle_tree = MerkleTree::new(merkle_tree_height);
        
        // Manually serialize merkle tree data 
        let mut merkle_tree_data = vec![];
        merkle_tree_data.push(merkle_tree.is_initialized as u8);
        merkle_tree_data.push(merkle_tree.height);
        merkle_tree_data.extend_from_slice(&merkle_tree.current_index.to_le_bytes());
        merkle_tree_data.extend_from_slice(&merkle_tree.root);
        
        // Serialize filled_subtrees
        let subtrees_len = merkle_tree.filled_subtrees.len() as u32;
        merkle_tree_data.extend_from_slice(&subtrees_len.to_le_bytes());
        
        for subtree in &merkle_tree.filled_subtrees {
            merkle_tree_data.extend_from_slice(subtree);
        }
        
        self.accounts.insert(merkle_tree_pda, merkle_tree_data);
        
        Ok(())
    }
    
    /// Shield tokens by adding a commitment to the merkle tree
    pub fn shield(&mut self, amount: u64, commitment: [u8; 32]) -> Result<(), PrivacyError> {
        // Get the PDAs
        let (pool_pda, _) = find_pool_pda(&self.program_id);
        let (merkle_tree_pda, _) = find_merkle_tree_pda(&self.program_id);
        
        // Get the pool
        let pool_data = self.accounts.get(&pool_pda).ok_or(PrivacyError::InvalidPool)?;
        let mut pool = match Pool::try_from_slice(pool_data) {
            Ok(pool) => pool,
            Err(_) => return Err(PrivacyError::InvalidPool),
        };
        
        // Make sure the pool is initialized
        if !pool.is_initialized {
            return Err(PrivacyError::InvalidPool);
        }
        
        // Get the merkle tree
        let merkle_tree_data = self.accounts.get(&merkle_tree_pda).ok_or(PrivacyError::InvalidPool)?;
        let mut merkle_tree = match MerkleTree::try_from_slice(merkle_tree_data) {
            Ok(tree) => tree,
            Err(_) => return Err(PrivacyError::InvalidPool),
        };
        
        // Make sure the merkle tree is initialized
        if !merkle_tree.is_initialized {
            return Err(PrivacyError::InvalidPool);
        }
        
        // Insert the commitment into the merkle tree
        merkle_tree.insert(&commitment);
        
        // Update the pool's total amount
        pool.total_amount += amount;
        
        // Manually serialize the pool
        let mut updated_pool_data = vec![];
        updated_pool_data.push(pool.is_initialized as u8);
        updated_pool_data.push(pool.merkle_tree_height);
        updated_pool_data.extend_from_slice(&pool.total_amount.to_le_bytes());
        
        // Manually serialize the merkle tree
        let mut updated_tree_data = vec![];
        updated_tree_data.push(merkle_tree.is_initialized as u8);
        updated_tree_data.push(merkle_tree.height);
        updated_tree_data.extend_from_slice(&merkle_tree.current_index.to_le_bytes());
        updated_tree_data.extend_from_slice(&merkle_tree.root);
        
        // Serialize filled_subtrees
        let subtrees_len = merkle_tree.filled_subtrees.len() as u32;
        updated_tree_data.extend_from_slice(&subtrees_len.to_le_bytes());
        
        for subtree in &merkle_tree.filled_subtrees {
            updated_tree_data.extend_from_slice(subtree);
        }
        
        // Update the accounts
        self.accounts.insert(pool_pda, updated_pool_data);
        self.accounts.insert(merkle_tree_pda, updated_tree_data);
        
        Ok(())
    }
    
    /// Withdraw tokens by proving you know a valid nullifier
    pub fn withdraw(
        &mut self, 
        amount: u64, 
        root: [u8; 32], 
        nullifier_hash: [u8; 32],
        _proof: Vec<u8>, // Changed from [u8; 256] to Vec<u8>
        destination: &Pubkey,
    ) -> Result<(), PrivacyError> {
        // Get the PDAs
        let (pool_pda, _) = find_pool_pda(&self.program_id);
        let (merkle_tree_pda, _) = find_merkle_tree_pda(&self.program_id);
        let (nullifier_pda, _) = find_nullifier_pda(&self.program_id, &nullifier_hash);
        
        // Get the pool
        let pool_data = self.accounts.get(&pool_pda).ok_or(PrivacyError::InvalidPool)?;
        let mut pool = match Pool::try_from_slice(pool_data) {
            Ok(pool) => pool,
            Err(_) => return Err(PrivacyError::InvalidPool),
        };
        
        // Make sure the pool is initialized
        if !pool.is_initialized {
            return Err(PrivacyError::InvalidPool);
        }
        
        // Get the merkle tree
        let merkle_tree_data = self.accounts.get(&merkle_tree_pda).ok_or(PrivacyError::InvalidPool)?;
        let merkle_tree = match MerkleTree::try_from_slice(merkle_tree_data) {
            Ok(tree) => tree,
            Err(_) => return Err(PrivacyError::InvalidPool),
        };
        
        // Make sure the merkle tree is initialized
        if !merkle_tree.is_initialized {
            return Err(PrivacyError::InvalidPool);
        }
        
        // Check that the root is valid
        if merkle_tree.root != root {
            return Err(PrivacyError::InvalidRoot);
        }
        
        // Check if the nullifier has been used before
        if self.accounts.contains_key(&nullifier_pda) {
            return Err(PrivacyError::NullifierAlreadyUsed);
        }
        
        // Check if there are enough funds in the pool
        if pool.total_amount < amount {
            return Err(PrivacyError::InsufficientFunds);
        }
        
        // Save the nullifier as used
        let nullifier = Nullifier {
            is_initialized: true,
            nullifier_hash,
        };
        
        // Manually serialize nullifier
        let mut nullifier_data = vec![];
        nullifier_data.push(nullifier.is_initialized as u8);
        nullifier_data.extend_from_slice(&nullifier.nullifier_hash);
        
        // Reduce the pool's total amount
        pool.total_amount -= amount;
        
        // Manually serialize the updated pool
        let mut updated_pool_data = vec![];
        updated_pool_data.push(pool.is_initialized as u8);
        updated_pool_data.push(pool.merkle_tree_height);
        updated_pool_data.extend_from_slice(&pool.total_amount.to_le_bytes());
        
        // Add the amount to the destination (for tests we don't track this, but it would happen in the real system)
        println!("Sent {} lamports to {}", amount, destination);
        
        // Update the accounts
        self.accounts.insert(pool_pda, updated_pool_data);
        self.accounts.insert(nullifier_pda, nullifier_data);
        
        Ok(())
    }
    
    /// Get a copy of an account from the bank
    pub fn get_account<T: BorshDeserialize>(&self, pubkey: &Pubkey) -> Option<T> {
        self.accounts.get(pubkey).and_then(|data| T::try_from_slice(data).ok())
    }
    
    /// Get the raw data of an account from the bank
    pub fn get_account_data(&self, pubkey: &Pubkey) -> Option<Vec<u8>> {
        self.accounts.get(pubkey).cloned()
    }
    
    /// Check if an account exists
    pub fn account_exists(&self, pubkey: &Pubkey) -> bool {
        self.accounts.contains_key(pubkey)
    }
} 