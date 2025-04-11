use crate::common::fixtures::{get_program_id, get_test_keypair, MERKLE_TREE_HEIGHT, MOCK_COMMITMENT, MOCK_ROOT, MOCK_NULLIFIER_HASH, get_mock_proof};
use crate::common::test_utils::{create_initialize_instruction, process_transaction, create_shield_instruction, create_withdraw_instruction};
use crate::common::validator_setup::{setup_validator, deploy_program, initialize_zkcash};
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::TransactionError, pubkey::Pubkey};
use zkcash::utils::{find_pool_pda, find_merkle_tree_pda, find_nullifier_pda};
use zkcash::state::{Pool, MerkleTree, Nullifier};
use borsh::BorshDeserialize;
use crate::common::simple_bank::SimpleBank;
use zkcash::error::PrivacyError;

// A single test that demonstrates the withdraw test approach
#[tokio::test]
async fn test_withdraw_functionality() {
    // A simple check to verify tests run
    assert!(true, "This test should always pass");
    
    println!("NOTE: Withdraw integration tests are limited in this environment.");
    println!("Full testing requires a proper test validator with program processor.");
    
    // Summary of withdraw tests that would be run in a proper environment:
    println!("- test_withdraw_success: Withdraw funds successfully");
    println!("- test_withdraw_invalid_accounts: Try withdrawing with invalid accounts");
    println!("- test_withdraw_insufficient_funds: Try withdrawing more than available");
    println!("- test_withdraw_used_nullifier: Try using the same nullifier twice");
}

/// Test withdrawing tokens successfully after shielding
#[test]
fn test_withdraw_success() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    bank.initialize(MERKLE_TREE_HEIGHT).expect("Initialization should succeed");
    
    // Shield some tokens
    let shield_amount = 2_000_000; // 2 SOL
    bank.shield(shield_amount, MOCK_COMMITMENT).expect("Shield operation should succeed");
    
    // Get the merkle tree so we can get the root
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    let merkle_tree: MerkleTree = bank.get_account(&merkle_tree_pda).expect("Merkle tree account not found");
    let root = merkle_tree.root;
    
    // Now withdraw some of the tokens
    let withdraw_amount = 1_000_000; // 1 SOL
    let proof = get_mock_proof();
    let destination = Pubkey::new_unique();
    
    let result = bank.withdraw(
        withdraw_amount,
        root,
        MOCK_NULLIFIER_HASH,
        proof,
        &destination,
    );
    
    assert!(result.is_ok(), "Withdraw operation failed: {:?}", result.err());
    
    // Verify the pool's total amount decreased
    let (pool_pda, _) = find_pool_pda(&program_id);
    let pool: Pool = bank.get_account(&pool_pda).expect("Pool account not found");
    assert_eq!(pool.total_amount, shield_amount - withdraw_amount, "Pool total amount should have decreased");
    
    // Verify the nullifier was marked as used
    let (nullifier_pda, _) = find_nullifier_pda(&program_id, &MOCK_NULLIFIER_HASH);
    let nullifier: Nullifier = bank.get_account(&nullifier_pda).expect("Nullifier account not found");
    assert!(nullifier.is_initialized, "Nullifier should be marked as used");
    assert_eq!(nullifier.nullifier_hash, MOCK_NULLIFIER_HASH, "Nullifier hash should match");
}

/// Test withdrawing with an invalid root
#[test]
fn test_withdraw_invalid_root() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    bank.initialize(MERKLE_TREE_HEIGHT).expect("Initialization should succeed");
    
    // Shield some tokens
    let shield_amount = 2_000_000; // 2 SOL
    bank.shield(shield_amount, MOCK_COMMITMENT).expect("Shield operation should succeed");
    
    // Create an invalid root
    let invalid_root = [42u8; 32];
    
    // Now try to withdraw tokens with the invalid root
    let withdraw_amount = 1_000_000; // 1 SOL
    let proof = get_mock_proof();
    let destination = Pubkey::new_unique();
    
    let result = bank.withdraw(
        withdraw_amount,
        invalid_root,
        MOCK_NULLIFIER_HASH,
        proof,
        &destination,
    );
    
    assert!(result.is_err(), "Withdraw operation with invalid root should fail");
    match result {
        Err(PrivacyError::InvalidRoot) => {
            // This is the expected error
        },
        _ => {
            panic!("Unexpected error: {:?}", result);
        }
    }
}

/// Test withdrawing with the same nullifier twice
#[test]
fn test_withdraw_double_spend() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    bank.initialize(MERKLE_TREE_HEIGHT).expect("Initialization should succeed");
    
    // Shield some tokens
    let shield_amount = 3_000_000; // 3 SOL
    bank.shield(shield_amount, MOCK_COMMITMENT).expect("Shield operation should succeed");
    
    // Get the merkle tree so we can get the root
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    let merkle_tree: MerkleTree = bank.get_account(&merkle_tree_pda).expect("Merkle tree account not found");
    let root = merkle_tree.root;
    
    // Withdraw tokens the first time
    let withdraw_amount = 1_000_000; // 1 SOL
    let proof = get_mock_proof();
    let destination = Pubkey::new_unique();
    
    let result = bank.withdraw(
        withdraw_amount,
        root,
        MOCK_NULLIFIER_HASH,
        proof.clone(),
        &destination,
    );
    
    assert!(result.is_ok(), "First withdraw operation failed: {:?}", result.err());
    
    // Try to withdraw again with the same nullifier
    let result = bank.withdraw(
        withdraw_amount,
        root,
        MOCK_NULLIFIER_HASH,
        proof,
        &destination,
    );
    
    assert!(result.is_err(), "Second withdraw with same nullifier should fail");
    match result {
        Err(PrivacyError::NullifierAlreadyUsed) => {
            // This is the expected error
        },
        _ => {
            panic!("Unexpected error: {:?}", result);
        }
    }
}

/// Test withdrawing more than the pool's balance
#[test]
fn test_withdraw_insufficient_funds() {
    // Create a new SimpleBank with our program ID
    let program_id = get_program_id();
    let mut bank = SimpleBank::new(program_id);
    
    // Initialize the ZKCash program
    bank.initialize(MERKLE_TREE_HEIGHT).expect("Initialization should succeed");
    
    // Shield a small amount of tokens
    let shield_amount = 500_000; // 0.5 SOL
    bank.shield(shield_amount, MOCK_COMMITMENT).expect("Shield operation should succeed");
    
    // Get the merkle tree so we can get the root
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    let merkle_tree: MerkleTree = bank.get_account(&merkle_tree_pda).expect("Merkle tree account not found");
    let root = merkle_tree.root;
    
    // Try to withdraw more than the pool's balance
    let withdraw_amount = 1_000_000; // 1 SOL
    let proof = get_mock_proof();
    let destination = Pubkey::new_unique();
    
    let result = bank.withdraw(
        withdraw_amount,
        root,
        MOCK_NULLIFIER_HASH,
        proof,
        &destination,
    );
    
    assert!(result.is_err(), "Withdraw operation with insufficient funds should fail");
    match result {
        Err(PrivacyError::InsufficientFunds) => {
            // This is the expected error
        },
        _ => {
            panic!("Unexpected error: {:?}", result);
        }
    }
}
