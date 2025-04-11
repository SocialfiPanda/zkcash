use solana_sdk::pubkey::Pubkey;
use crate::common::fixtures::{get_program_id, MERKLE_TREE_HEIGHT, MOCK_COMMITMENT};
use crate::common::simple_bank::SimpleBank;
use zkcash::utils::{find_pool_pda, find_merkle_tree_pda};
use zkcash::state::{Pool, MerkleTree};
use zkcash::error::PrivacyError;

/// Test shielding tokens successfully
#[test]
fn test_shield_success() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    bank.initialize(MERKLE_TREE_HEIGHT).expect("Initialization should succeed");
    
    // Shield some tokens
    let amount = 1_000_000; // 1 SOL
    let result = bank.shield(amount, MOCK_COMMITMENT);
    assert!(result.is_ok(), "Shield operation failed: {:?}", result.err());
    
    // Verify the pool's total amount increased
    let (pool_pda, _) = find_pool_pda(&program_id);
    let pool: Pool = bank.get_account(&pool_pda).expect("Pool account not found");
    assert_eq!(pool.total_amount, amount, "Pool total amount should have increased");
    
    // Verify the commitment was added to the merkle tree
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    let merkle_tree: MerkleTree = bank.get_account(&merkle_tree_pda).expect("Merkle tree account not found");
    assert_eq!(merkle_tree.current_index, 1, "Merkle tree index should have increased");
}

/// Test shielding tokens to an uninitialized pool
#[test]
fn test_shield_uninitialized_pool() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Do NOT initialize the ZKCash program
    
    // Try to shield some tokens
    let amount = 1_000_000; // 1 SOL
    let result = bank.shield(amount, MOCK_COMMITMENT);
    
    // Verify the operation failed with the expected error
    assert!(result.is_err(), "Shield operation should have failed");
    match result {
        Err(PrivacyError::InvalidPool) => {
            // This is the expected error
        },
        _ => {
            panic!("Unexpected error: {:?}", result);
        }
    }
}

/// Test shielding tokens multiple times
#[test]
fn test_shield_multiple() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    bank.initialize(MERKLE_TREE_HEIGHT).expect("Initialization should succeed");
    
    // Shield some tokens multiple times
    let amount1 = 1_000_000; // 1 SOL
    let amount2 = 2_000_000; // 2 SOL
    
    // Create different commitments
    let commitment1 = MOCK_COMMITMENT;
    let mut commitment2 = [0u8; 32];
    commitment2.copy_from_slice(&[2; 32]); // Different commitment
    
    // Shield first amount
    bank.shield(amount1, commitment1).expect("First shield operation should succeed");
    
    // Shield second amount
    bank.shield(amount2, commitment2).expect("Second shield operation should succeed");
    
    // Verify the pool's total amount increased by both amounts
    let (pool_pda, _) = find_pool_pda(&program_id);
    let pool: Pool = bank.get_account(&pool_pda).expect("Pool account not found");
    assert_eq!(pool.total_amount, amount1 + amount2, "Pool total amount should have increased by both amounts");
    
    // Verify the merkle tree index increased for both insertions
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    let merkle_tree: MerkleTree = bank.get_account(&merkle_tree_pda).expect("Merkle tree account not found");
    assert_eq!(merkle_tree.current_index, 2, "Merkle tree index should have increased for both insertions");
}
