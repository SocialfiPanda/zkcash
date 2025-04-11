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
        
        // Call the function
        let (pool_pda, bump_seed) = find_pool_pda(&program_id);
        
        // Verify the PDA is derived correctly
        let seeds = &[
            b"privacy_pool",
            program_id.as_ref(),
            &[bump_seed],
        ];
        let (expected_pda, _) = Pubkey::find_program_address(seeds, &program_id);
        assert_eq!(pool_pda, expected_pda);
    }
    
    /// Test finding a merkle tree PDA
    #[test]
    fn test_find_merkle_tree_pda() {
        // Create a program ID
        let program_id = Pubkey::new_unique();
        
        // Call the function
        let (merkle_tree_pda, bump_seed) = find_merkle_tree_pda(&program_id);
        
        // Verify the PDA is derived correctly
        let seeds = &[
            b"merkle_tree",
            program_id.as_ref(),
            &[bump_seed],
        ];
        let (expected_pda, _) = Pubkey::find_program_address(seeds, &program_id);
        assert_eq!(merkle_tree_pda, expected_pda);
    }
    
    /// Test finding a nullifier PDA
    #[test]
    fn test_find_nullifier_pda() {
        // Create a program ID
        let program_id = Pubkey::new_unique();
        
        // Call the function
        let (nullifier_pda, bump_seed) = find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
        
        // Verify the PDA is derived correctly
        let seeds = &[
            b"nullifier",
            MOCK_NULLIFIER_HASH.as_ref(),
            &[bump_seed],
        ];
        let (expected_pda, _) = Pubkey::find_program_address(seeds, &program_id);
        assert_eq!(nullifier_pda, expected_pda);
    }
    
    /// Test utilities class methods
    #[test]
    fn test_utils_class() {
        let utils = Utils::new();
        
        // Create a program ID
        let program_id = Pubkey::new_unique();
        
        // Test find_pool_pda
        let (pool_pda, _) = find_pool_pda(&program_id);
        let (utils_pool_pda, _) = utils.find_pool_pda(&program_id);
        assert_eq!(pool_pda, utils_pool_pda);
        
        // Test find_merkle_tree_pda
        let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
        let (utils_merkle_tree_pda, _) = utils.find_merkle_tree_pda(&program_id);
        assert_eq!(merkle_tree_pda, utils_merkle_tree_pda);
        
        // Test find_nullifier_pda
        let (nullifier_pda, _) = find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
        let (utils_nullifier_pda, _) = utils.find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
        assert_eq!(nullifier_pda, utils_nullifier_pda);
    }
}
