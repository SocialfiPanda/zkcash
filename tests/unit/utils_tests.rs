use solana_program::pubkey::Pubkey;
use crate::common::fixtures::MOCK_NULLIFIER_HASH;
use zkcash::utils::{find_pool_pda, find_merkle_tree_pda, find_nullifier_pda, Utils};

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test finding a pool PDA
    #[test]
    fn test_find_pool_pda() {
        // Create a program ID
        let program_id = Pubkey::new_unique();
        
        // Call the function twice
        let (pool_pda1, bump_seed1) = find_pool_pda(&program_id);
        let (pool_pda2, bump_seed2) = find_pool_pda(&program_id);
        
        // Verify the function returns deterministic results
        assert_eq!(pool_pda1, pool_pda2);
        assert_eq!(bump_seed1, bump_seed2);
    }
    
    /// Test finding a merkle tree PDA
    #[test]
    fn test_find_merkle_tree_pda() {
        // Create a program ID
        let program_id = Pubkey::new_unique();
        
        // Call the function twice
        let (merkle_tree_pda1, bump_seed1) = find_merkle_tree_pda(&program_id);
        let (merkle_tree_pda2, bump_seed2) = find_merkle_tree_pda(&program_id);
        
        // Verify the function returns deterministic results
        assert_eq!(merkle_tree_pda1, merkle_tree_pda2);
        assert_eq!(bump_seed1, bump_seed2);
    }
    
    /// Test finding a nullifier PDA
    #[test]
    fn test_find_nullifier_pda() {
        // Create a program ID
        let program_id = Pubkey::new_unique();
        
        // Call the function twice
        let (nullifier_pda1, bump_seed1) = find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
        let (nullifier_pda2, bump_seed2) = find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
        
        // Verify the function returns deterministic results
        assert_eq!(nullifier_pda1, nullifier_pda2);
        assert_eq!(bump_seed1, bump_seed2);
    }
    
    /// Test utilities class methods
    #[test]
    fn test_utils_class() {
        let utils = Utils::new();
        
        // Create a program ID
        let program_id = Pubkey::new_unique();
        
        // Test find_pool_pda
        let (pool_pda1, bump_seed1) = utils.find_pool_pda(&program_id);
        let (pool_pda2, bump_seed2) = utils.find_pool_pda(&program_id);
        assert_eq!(pool_pda1, pool_pda2);
        assert_eq!(bump_seed1, bump_seed2);
        
        // Test find_merkle_tree_pda
        let (merkle_tree_pda1, bump_seed1) = utils.find_merkle_tree_pda(&program_id);
        let (merkle_tree_pda2, bump_seed2) = utils.find_merkle_tree_pda(&program_id);
        assert_eq!(merkle_tree_pda1, merkle_tree_pda2);
        assert_eq!(bump_seed1, bump_seed2);
        
        // Test find_nullifier_pda
        let (nullifier_pda1, bump_seed1) = utils.find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
        let (nullifier_pda2, bump_seed2) = utils.find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
        assert_eq!(nullifier_pda1, nullifier_pda2);
        assert_eq!(bump_seed1, bump_seed2);
    }
}
