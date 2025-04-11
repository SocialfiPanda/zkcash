use crate::common::fixtures::{get_program_id, get_test_keypair, MERKLE_TREE_HEIGHT, MOCK_COMMITMENT};
use crate::common::test_utils::{create_and_start_program, process_transaction, create_initialize_instruction, create_shield_instruction};
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::TransactionError};
use zkcash::utils::{find_pool_pda, find_merkle_tree_pda};

#[tokio::test]
async fn test_shield_success() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Initialize the program first
    let init_instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        MERKLE_TREE_HEIGHT,
    );
    
    // Process the initialization transaction
    let result = process_transaction(&mut context, &[init_instruction], &[&payer]).await;
    assert!(result.is_ok());
    
    // Get the initial pool balance
    let initial_pool_account = context.banks_client.get_account(pool_pda).await.unwrap().unwrap();
    let initial_pool: zkcash::state::Pool = borsh::BorshDeserialize::try_from_slice(&initial_pool_account.data).unwrap();
    let initial_amount = initial_pool.total_amount;
    
    // Get the initial merkle tree state
    let initial_merkle_tree_account = context.banks_client.get_account(merkle_tree_pda).await.unwrap().unwrap();
    let initial_merkle_tree: zkcash::state::MerkleTree = borsh::BorshDeserialize::try_from_slice(&initial_merkle_tree_account.data).unwrap();
    let initial_index = initial_merkle_tree.current_index;
    let initial_root = initial_merkle_tree.root;
    
    // Create the shield instruction
    let amount = 1000000;
    let shield_instruction = create_shield_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        amount,
        MOCK_COMMITMENT,
    );
    
    // Process the shield transaction
    let result = process_transaction(&mut context, &[shield_instruction], &[&payer]).await;
    
    // Verify the transaction was successful
    assert!(result.is_ok());
    
    // Get the updated pool state
    let updated_pool_account = context.banks_client.get_account(pool_pda).await.unwrap().unwrap();
    let updated_pool: zkcash::state::Pool = borsh::BorshDeserialize::try_from_slice(&updated_pool_account.data).unwrap();
    
    // Verify the pool amount was increased
    assert_eq!(updated_pool.total_amount, initial_amount + amount);
    
    // Get the updated merkle tree state
    let updated_merkle_tree_account = context.banks_client.get_account(merkle_tree_pda).await.unwrap().unwrap();
    let updated_merkle_tree: zkcash::state::MerkleTree = borsh::BorshDeserialize::try_from_slice(&updated_merkle_tree_account.data).unwrap();
    
    // Verify the merkle tree was updated
    assert_eq!(updated_merkle_tree.current_index, initial_index + 1);
    assert_ne!(updated_merkle_tree.root, initial_root);
}

#[tokio::test]
async fn test_shield_invalid_accounts() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Initialize the program first
    let init_instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        MERKLE_TREE_HEIGHT,
    );
    
    // Process the initialization transaction
    let result = process_transaction(&mut context, &[init_instruction], &[&payer]).await;
    assert!(result.is_ok());
    
    // Create a shield instruction with an invalid pool account
    let invalid_pool = solana_sdk::pubkey::Pubkey::new_unique();
    let amount = 1000000;
    let shield_instruction = create_shield_instruction(
        &program_id,
        &payer.pubkey(),
        &invalid_pool, // Invalid pool PDA
        &merkle_tree_pda,
        amount,
        MOCK_COMMITMENT,
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[shield_instruction], &[&payer]).await;
    
    // Verify the transaction failed
    assert!(result.is_err());
    
    // Create a shield instruction with an invalid merkle tree account
    let invalid_merkle_tree = solana_sdk::pubkey::Pubkey::new_unique();
    let shield_instruction = create_shield_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &invalid_merkle_tree, // Invalid merkle tree PDA
        amount,
        MOCK_COMMITMENT,
    );
    
    // Process the transaction
    let result = process_transaction(&mut context, &[shield_instruction], &[&payer]).await;
    
    // Verify the transaction failed
    assert!(result.is_err());
}

#[tokio::test]
async fn test_shield_without_initialization() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Create the shield instruction without initializing first
    let amount = 1000000;
    let shield_instruction = create_shield_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        amount,
        MOCK_COMMITMENT,
    );
    
    // Process the shield transaction
    let result = process_transaction(&mut context, &[shield_instruction], &[&payer]).await;
    
    // Verify the transaction failed
    assert!(result.is_err());
}

#[tokio::test]
async fn test_shield_multiple_times() {
    // Get program ID and test keypair
    let program_id = get_program_id();
    let payer = get_test_keypair();
    
    // Find PDAs
    let (pool_pda, _) = find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = find_merkle_tree_pda(&program_id);
    
    // Create and start the program test
    let (mut context, _) = create_and_start_program(&[], &program_id).await;
    
    // Initialize the program first
    let init_instruction = create_initialize_instruction(
        &program_id,
        &payer.pubkey(),
        &pool_pda,
        &merkle_tree_pda,
        MERKLE_TREE_HEIGHT,
    );
    
    // Process the initialization transaction
    let result = process_transaction(&mut context, &[init_instruction], &[&payer]).await;
    assert!(result.is_ok());
    
    // Shield multiple times
    let num_shields = 3;
    let amount = 1000000;
    
    for i in 0..num_shields {
        // Create a unique commitment for each shield
        let mut commitment = MOCK_COMMITMENT;
        commitment[0] = i as u8;
        
        // Create the shield instruction
        let shield_instruction = create_shield_instruction(
            &program_id,
            &payer.pubkey(),
            &pool_pda,
            &merkle_tree_pda,
            amount,
            commitment,
        );
        
        // Process the shield transaction
        let result = process_transaction(&mut context, &[shield_instruction], &[&payer]).await;
        
        // Verify the transaction was successful
        assert!(result.is_ok());
    }
    
    // Get the final pool state
    let final_pool_account = context.banks_client.get_account(pool_pda).await.unwrap().unwrap();
    let final_pool: zkcash::state::Pool = borsh::BorshDeserialize::try_from_slice(&final_pool_account.data).unwrap();
    
    // Verify the pool amount was increased by the total amount shielded
    assert_eq!(final_pool.total_amount, amount * num_shields);
    
    // Get the final merkle tree state
    let final_merkle_tree_account = context.banks_client.get_account(merkle_tree_pda).await.unwrap().unwrap();
    let final_merkle_tree: zkcash::state::MerkleTree = borsh::BorshDeserialize::try_from_slice(&final_merkle_tree_account.data).unwrap();
    
    // Verify the merkle tree index was increased by the number of shields
    assert_eq!(final_merkle_tree.current_index, num_shields);
}
