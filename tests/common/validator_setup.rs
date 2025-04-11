use solana_program::pubkey::Pubkey;
use solana_program_test::{ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::{Account, AccountSharedData, ReadableAccount, WritableAccount},
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
    rent::Rent,
};
use std::str::FromStr;
use std::path::PathBuf;
use std::fs;
use borsh::{BorshSerialize};
use zkcash::state::{Pool, MerkleTree};

/// Set up a mock test environment that doesn't rely on the actual program processor
pub async fn setup_validator() -> Result<ProgramTestContext, TransportError> {
    // Get program ID from fixtures
    let program_id = crate::common::fixtures::get_program_id();
    
    // Create a ProgramTest with just the program ID
    let mut program_test = ProgramTest::new(
        "zkcash",  // Program name
        program_id, // Program ID
        None,      // No processor provided
    );
    
    // We need to set up PDAs manually since we don't have the actual program processor
    let (pool_pda, _) = zkcash::utils::find_pool_pda(&program_id);
    let (merkle_tree_pda, _) = zkcash::utils::find_merkle_tree_pda(&program_id);
    
    // Create empty pool account
    let pool = Pool {
        is_initialized: false,
        merkle_tree_height: 0,
        total_amount: 0,
    };
    
    // Manually serialize pool data since borsh::BorshSerialize is not implemented for Pool
    let mut pool_data = vec![];
    pool_data.extend_from_slice(&[pool.is_initialized as u8]);
    pool_data.push(pool.merkle_tree_height);
    pool_data.extend_from_slice(&pool.total_amount.to_le_bytes());
    
    let pool_lamports = Rent::default().minimum_balance(pool_data.len());
    let mut pool_account = Account::new(
        pool_lamports,
        pool_data.len(),
        &program_id,
    );
    pool_account.data = pool_data;
    
    program_test.add_account(pool_pda, pool_account);
    
    // Create empty merkle tree account
    let merkle_tree = MerkleTree::new(crate::common::fixtures::MERKLE_TREE_HEIGHT);
    
    // Manually serialize merkle tree data 
    let mut merkle_tree_data = vec![];
    merkle_tree_data.extend_from_slice(&[merkle_tree.is_initialized as u8]);
    merkle_tree_data.push(merkle_tree.height);
    merkle_tree_data.extend_from_slice(&merkle_tree.current_index.to_le_bytes());
    merkle_tree_data.extend_from_slice(&merkle_tree.root);
    
    // Serialize filled_subtrees
    let subtrees_len = merkle_tree.filled_subtrees.len() as u32;
    merkle_tree_data.extend_from_slice(&subtrees_len.to_le_bytes());
    
    for subtree in &merkle_tree.filled_subtrees {
        merkle_tree_data.extend_from_slice(subtree);
    }
    
    let merkle_tree_lamports = Rent::default().minimum_balance(merkle_tree_data.len());
    let mut merkle_tree_account = Account::new(
        merkle_tree_lamports,
        merkle_tree_data.len(),
        &program_id,
    );
    merkle_tree_account.data = merkle_tree_data;
    
    program_test.add_account(merkle_tree_pda, merkle_tree_account);
    
    // Configure the validator
    program_test.prefer_bpf(false); // Don't use BPF since we're mocking
    
    // Start the test context
    let context = program_test.start_with_context().await;
    
    Ok(context)
}

/// Mock program deployment - since we're just using test accounts,
/// we don't actually need to deploy a program
pub async fn deploy_program(context: &mut ProgramTestContext) -> Result<(), TransportError> {
    println!("Mock program deployment (no actual deployment needed)");
    Ok(())
}

/// Mock initialization of the ZKCash program
pub async fn initialize_zkcash(context: &mut ProgramTestContext) -> Result<(), TransportError> {
    let program_id = crate::common::fixtures::get_program_id();
    
    // Find PDAs
    let (pool_pda, _) = zkcash::utils::find_pool_pda(&program_id);
    
    // Get the current account
    let pool_account = context.banks_client.get_account(pool_pda).await?.unwrap();
    
    // Create initialized pool data
    let pool = Pool {
        is_initialized: true,
        merkle_tree_height: crate::common::fixtures::MERKLE_TREE_HEIGHT,
        total_amount: 0,
    };
    
    // Manually serialize pool data
    let mut pool_data = vec![];
    pool_data.extend_from_slice(&[pool.is_initialized as u8]);
    pool_data.push(pool.merkle_tree_height);
    pool_data.extend_from_slice(&pool.total_amount.to_le_bytes());

    // Create a new Shared Account
    let mut new_account = AccountSharedData::new(
        pool_account.lamports(), 
        pool_data.len(),
        pool_account.owner(),
    );
    
    // Update the data manually
    new_account.data_as_mut_slice().copy_from_slice(&pool_data);
    
    // Set the account back
    context.set_account(&pool_pda, &new_account);
    
    println!("ZKCash program mock initialized successfully");
    
    Ok(())
}

/// Process a transaction that would normally be handled by the program
/// In the mock environment, we update the state directly
pub async fn mock_process_instruction(
    context: &mut ProgramTestContext,
    instruction_type: &str,
    accounts: &[(&Pubkey, Option<&[u8]>)],
    params: &[(&str, &dyn std::fmt::Debug)],
) -> Result<(), TransportError> {
    println!("Processing mock instruction: {}", instruction_type);
    
    for (account, data_opt) in accounts {
        if let Some(data) = data_opt {
            if let Some(account_data) = context.banks_client.get_account(**account).await? {
                // Create a new account with the same properties
                let mut new_account = AccountSharedData::new(
                    account_data.lamports(),
                    data.len(),
                    account_data.owner(),
                );
                
                // Update the data
                new_account.data_as_mut_slice().copy_from_slice(data);
                
                // Set the account back
                context.set_account(account, &new_account);
            }
        }
    }
    
    for (name, value) in params {
        println!(" - {}: {:?}", name, value);
    }
    
    Ok(())
}

/// Find the ZKCash program binary in the target directory
fn find_program_binary() -> Option<PathBuf> {
    let possible_paths = [
        // Standard debug build path
        "../target/deploy/zkcash.so",
        // Standard release build path
        "../target/bpfel-unknown-unknown/release/zkcash.so",
        // Alternative paths
        "target/deploy/zkcash.so",
        "target/bpfel-unknown-unknown/release/zkcash.so",
    ];
    
    for path in possible_paths.iter() {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }
    
    None
}

/// Helper function to add PDAs to fixtures module
pub fn add_pdas_to_fixtures() {
    // This is a placeholder. In a real implementation, you might
    // add code to register PDAs with the fixture module if needed.
} 