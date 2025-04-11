use crate::common::fixtures::{get_program_id, get_test_keypair, MERKLE_TREE_HEIGHT};
use crate::common::test_utils::{create_and_start_program, process_transaction, create_initialize_instruction};
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::TransactionError};
use zkcash::utils::{find_pool_pda, find_merkle_tree_pda};

#[tokio::test]
async fn test_initialize_success() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Create the initialize instruction
    let instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        MERKLE_TREE_HEIGHT,
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[instruction], &[&payer]).await;
    
    // Verify the transaction was successful
    assert!(result.is_ok());
    
    // Verify the accounts were created
    let pool_account = context.banks_client.get_account(pool_pda).await.unwrap().unwrap();
    let merkle_tree_account = context.banks_client.get_account(merkle_tree_pda).await.unwrap().unwrap();
    
    // Verify the accounts are owned by the program
    assert_eq!(pool_account.owner, program_id);
    assert_eq!(merkle_tree_account.owner, program_id);
    
    // Deserialize the accounts to verify their state
    let pool = borsh::BorshDeserialize::try_from_slice(&pool_account.data).unwrap();
    let merkle_tree = borsh::BorshDeserialize::try_from_slice(&merkle_tree_account.data).unwrap();
    
    // Verify the pool state
    assert!(pool.is_initialized);
    assert_eq!(pool.merkle_tree_height, MERKLE_TREE_HEIGHT);
    assert_eq!(pool.total_amount, 0);
    
    // Verify the merkle tree state
    assert!(merkle_tree.is_initialized);
    assert_eq!(merkle_tree.height, MERKLE_TREE_HEIGHT);
    assert_eq!(merkle_tree.current_index, 0);
}

#[tokio::test]
async fn test_initialize_invalid_accounts() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Create an instruction with an invalid pool account
    let invalid_pool = solana_sdk::pubkey::Pubkey::new_unique();
    let instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &invalid_pool, // Invalid pool PDA
        &merkle_tree_pda,
        MERKLE_TREE_HEIGHT,
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[instruction], &[&payer]).await;
    
    // Verify the transaction failed
    assert!(result.is_err());
    
    // Create an instruction with an invalid merkle tree account
    let invalid_merkle_tree = solana_sdk::pubkey::Pubkey::new_unique();
    let instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &invalid_merkle_tree, // Invalid merkle tree PDA
        MERKLE_TREE_HEIGHT,
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[instruction], &[&payer]).await;
    
    // Verify the transaction failed
    assert!(result.is_err());
}

#[tokio::test]
async fn test_initialize_invalid_parameters() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Create an instruction with an invalid merkle tree height (too small)
    let instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        0, // Invalid height (too small)
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[instruction], &[&payer]).await;
    
    // Verify the transaction failed
    assert!(result.is_err());
    
    // Create an instruction with an invalid merkle tree height (too large)
    let instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        255, // Invalid height (too large)
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[instruction], &[&payer]).await;
    
    // Verify the transaction failed
    assert!(result.is_err());
}

#[tokio::test]
async fn test_initialize_twice() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Create the initialize instruction
    let instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        MERKLE_TREE_HEIGHT,
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[instruction], &[&payer]).await;
    
    // Verify the first initialization was successful
    assert!(result.is_ok());
    
    // Try to initialize again
    let result = process_transaction(&mut context, &[instruction], &[&payer]).await;
    
    // Verify the second initialization failed
    assert!(result.is_err());
    
    // Check if the error is because the account already exists
    match result.unwrap_err() {
        TransactionError::InstructionError(_, InstructionError::AccountAlreadyInitialized) => {
            // This is the expected error
        }
        TransactionError::InstructionError(_, InstructionError::Custom(error_code)) => {
            // This could be a custom error from the program
            assert_eq!(error_code, zkcash::error::PrivacyError::InvalidPool as u32);
        }
        err => {
            panic!("Unexpected error: {:?}", err);
        }
    }
}
