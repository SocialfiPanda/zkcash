use crate::common::fixtures::{get_program_id, MERKLE_TREE_HEIGHT};
use crate::common::simple_bank::SimpleBank;
use solana_sdk::pubkey::Pubkey;
use zkcash::utils::{find_pool_pda, find_merkle_tree_pda};
use zkcash::state::{Pool, MerkleTree};
use zkcash::error::PrivacyError;

/// Test initialization of the ZKCash program using our SimpleBank
#[test]
fn test_initialize_success() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    let result = bank.initialize(MERKLE_TREE_HEIGHT);
    assert!(result.is_ok(), "Initialization failed: {:?}", result.err());
    
    // Verify the accounts were properly initialized
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Read the pool account
    let pool: Pool = bank.get_account(&pool_pda).expect("Pool account not found");
    
    // Verify the pool is initialized
    assert!(pool.is_initialized, "Pool should be initialized");
    assert_eq!(pool.merkle_tree_height, MERKLE_TREE_HEIGHT);
    assert_eq!(pool.total_amount, 0);
    
    // Read the merkle tree account
    let merkle_tree: MerkleTree = bank.get_account(&merkle_tree_pda).expect("Merkle tree account not found");
    
    // Verify the merkle tree is initialized
    assert!(merkle_tree.is_initialized, "Merkle tree should be initialized");
    assert_eq!(merkle_tree.height, MERKLE_TREE_HEIGHT);
}

/// Test initialization with incorrect PDAs (in mock environment)
#[test]
fn test_initialize_with_incorrect_pdas() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    let result = bank.initialize(MERKLE_TREE_HEIGHT);
    assert!(result.is_ok(), "Initialization failed: {:?}", result.err());
    
    // Get the expected PDAs
    let (expected_pool_pda, _) = find_pool_pda(&program_id);
    let (expected_merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create random incorrect PDAs
    let incorrect_pool_pda = Pubkey::new_unique();
    let incorrect_merkle_tree_pda = Pubkey::new_unique();
    
    // Verify the incorrect PDAs are different from the expected ones
    assert_ne!(incorrect_pool_pda, expected_pool_pda, "Incorrect pool PDA should be different");
    assert_ne!(incorrect_merkle_tree_pda, expected_merkle_tree_pda, "Incorrect tree PDA should be different");
    
    // Verify the correct PDAs exist
    assert!(bank.account_exists(&expected_pool_pda), "Expected pool PDA account should exist");
    assert!(bank.account_exists(&expected_merkle_tree_pda), "Expected merkle tree PDA account should exist");
    
    // Verify the incorrect PDAs don't exist
    assert!(!bank.account_exists(&incorrect_pool_pda), "Incorrect pool PDA account should not exist");
    assert!(!bank.account_exists(&incorrect_merkle_tree_pda), "Incorrect merkle tree PDA account should not exist");
}

/// Test attempting to initialize the program twice
#[test]
fn test_initialize_twice() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // First initialization should succeed
    let result = bank.initialize(MERKLE_TREE_HEIGHT);
    assert!(result.is_ok(), "First initialization failed: {:?}", result.err());
    
    // Second initialization should fail
    let result = bank.initialize(MERKLE_TREE_HEIGHT);
    assert!(result.is_err(), "Second initialization unexpectedly succeeded");
    
    // Verify it's the right error
    match result {
        Err(PrivacyError::InvalidPool) => {
            // This is the expected error
        },
        _ => {
            panic!("Unexpected error: {:?}", result);
        }
    }
}
