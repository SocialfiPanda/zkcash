use solana_program::pubkey::Pubkey;
use solana_program_test::{ProgramTest, ProgramTestContext, BanksClientError};
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_program::program_pack::Pack;
use solana_program::system_instruction;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::sysvar;
use std::str::FromStr;
use zkcash::instruction::PrivacyInstruction;
use zkcash::state::{Pool, MerkleTree, Nullifier};
use borsh::BorshSerialize;

/// Setup a program test with the ZKCash program
pub fn setup_program_test() -> ProgramTest {
    let program_id = Pubkey::from_str("ZKCashProgramPubkey11111111111111111111111").unwrap();
    ProgramTest::new(
        "zkcash",
        program_id,
        None, // Use the processor directly
    )
}

/// Create and start a program test with specified accounts
pub async fn create_and_start_program(
    accounts: &[(Pubkey, usize)],
    program_id: &Pubkey,
) -> (ProgramTestContext, Keypair) {
    let mut program_test = setup_program_test();
    
    // Add accounts
    for (pubkey, size) in accounts {
        let account = Account::new(
            1000000000, // Rent exempt minimum
            *size,
            program_id,
        );
        program_test.add_account(*pubkey, account);
    }
    
    // Create a keypair for the payer
    let payer = Keypair::new();
    
    // Start the program test
    let context = program_test.start_with_context().await;
    
    // Airdrop some SOL to the payer
    context.banks_client.process_transaction(Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &context.payer.pubkey(),
            &payer.pubkey(),
            1000000000000,
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    )).await.unwrap();
    
    (context, payer)
}

/// Process a transaction with the given instructions
pub async fn process_transaction(
    context: &mut ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
) -> Result<(), BanksClientError> {
    let mut transaction = Transaction::new_with_payer(
        instructions,
        Some(&signers[0].pubkey()),
    );
    
    transaction.sign(signers, context.last_blockhash);
    
    context.banks_client.process_transaction(transaction).await.map_err(|e| e.into())
}

/// Create a pool account
pub async fn create_pool_account(
    context: &mut ProgramTestContext,
    program_id: &Pubkey,
    payer: &Keypair,
    pool_pda: &Pubkey,
    bump_seed: u8,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();
    let pool_size = std::mem::size_of::<Pool>();
    let pool_lamports = rent.minimum_balance(pool_size);
    
    let instruction = system_instruction::create_account(
        &payer.pubkey(),
        pool_pda,
        pool_lamports,
        pool_size as u64,
        program_id,
    );
    
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        context.last_blockhash,
    );
    
    context.banks_client.process_transaction(transaction).await.map_err(|e| e.into())
}

/// Create a merkle tree account
pub async fn create_merkle_tree_account(
    context: &mut ProgramTestContext,
    program_id: &Pubkey,
    payer: &Keypair,
    merkle_tree_pda: &Pubkey,
    bump_seed: u8,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();
    let merkle_tree_size = std::mem::size_of::<MerkleTree>();
    let merkle_tree_lamports = rent.minimum_balance(merkle_tree_size);
    
    let instruction = system_instruction::create_account(
        &payer.pubkey(),
        merkle_tree_pda,
        merkle_tree_lamports,
        merkle_tree_size as u64,
        program_id,
    );
    
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        context.last_blockhash,
    );
    
    context.banks_client.process_transaction(transaction).await.map_err(|e| e.into())
}

/// Create a nullifier account
pub async fn create_nullifier_account(
    context: &mut ProgramTestContext,
    program_id: &Pubkey,
    payer: &Keypair,
    nullifier_pda: &Pubkey,
    bump_seed: u8,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();
    let nullifier_size = std::mem::size_of::<Nullifier>();
    let nullifier_lamports = rent.minimum_balance(nullifier_size);
    
    let instruction = system_instruction::create_account(
        &payer.pubkey(),
        nullifier_pda,
        nullifier_lamports,
        nullifier_size as u64,
        program_id,
    );
    
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        context.last_blockhash,
    );
    
    context.banks_client.process_transaction(transaction).await.map_err(|e| e.into())
}

/// Create an initialize instruction
pub fn create_initialize_instruction(
    program_id: &Pubkey,
    payer: &Pubkey,
    pool_pda: &Pubkey,
    merkle_tree_pda: &Pubkey,
    merkle_tree_height: u8,
) -> Instruction {
    let data = PrivacyInstruction::Initialize {
        merkle_tree_height,
    }
    .try_to_vec()
    .unwrap();
    
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*pool_pda, false),
            AccountMeta::new(*merkle_tree_pda, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data,
    }
}

/// Create a shield instruction
pub fn create_shield_instruction(
    program_id: &Pubkey,
    payer: &Pubkey,
    pool_pda: &Pubkey,
    merkle_tree_pda: &Pubkey,
    amount: u64,
    commitment: [u8; 32],
) -> Instruction {
    let data = PrivacyInstruction::Shield {
        amount,
        commitment,
    }
    .try_to_vec()
    .unwrap();
    
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*pool_pda, false),
            AccountMeta::new(*merkle_tree_pda, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data,
    }
}

/// Create a withdraw instruction
pub fn create_withdraw_instruction(
    program_id: &Pubkey,
    payer: &Pubkey,
    pool_pda: &Pubkey,
    merkle_tree_pda: &Pubkey,
    nullifier_pda: &Pubkey,
    recipient: &Pubkey,
    amount: u64,
    root: [u8; 32],
    nullifier_hash: [u8; 32],
    proof: Vec<u8>,
) -> Instruction {
    let recipient_bytes = recipient.to_bytes();
    let mut recipient_array = [0u8; 32];
    recipient_array.copy_from_slice(&recipient_bytes);
    
    let data = PrivacyInstruction::Withdraw {
        amount,
        root,
        nullifier_hash,
        recipient: recipient_array,
        proof,
    }
    .try_to_vec()
    .unwrap();
    
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*pool_pda, false),
            AccountMeta::new(*merkle_tree_pda, false),
            AccountMeta::new(*nullifier_pda, false),
            AccountMeta::new(*recipient, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data,
    }
}
