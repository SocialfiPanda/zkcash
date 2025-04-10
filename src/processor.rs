use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke, invoke_signed},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use crate::{
    instruction::PrivacyInstruction,
    state::{Pool, MerkleTree, Nullifier},
    error::PrivacyError,
    utils::{find_pool_pda, find_merkle_tree_pda, find_nullifier_pda, Utils},
    verifier::Verifier,
};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = PrivacyInstruction::try_from_slice(instruction_data)?;
        
        match instruction {
            PrivacyInstruction::Initialize { merkle_tree_height } => {
                Self::process_initialize(program_id, accounts, merkle_tree_height)
            },
            PrivacyInstruction::Shield { amount, commitment } => {
                Self::process_shield(program_id, accounts, amount, commitment)
            },
            PrivacyInstruction::Withdraw { amount, root, nullifier_hash, recipient, proof } => {
                Self::process_withdraw(program_id, accounts, amount, root, nullifier_hash, recipient, proof)
            },
        }
    }
    
    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        merkle_tree_height: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        
        let payer_info = next_account_info(account_info_iter)?;
        let pool_info = next_account_info(account_info_iter)?;
        let merkle_tree_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;
        
        let (pool_pda, pool_bump) = find_pool_pda(program_id);
        if pool_pda != *pool_info.key {
            return Err(PrivacyError::InvalidPool.into());
        }
        
        let (merkle_tree_pda, merkle_tree_bump) = find_merkle_tree_pda(program_id);
        if merkle_tree_pda != *merkle_tree_info.key {
            return Err(ProgramError::InvalidAccountData);
        }
        
        let rent = Rent::from_account_info(rent_info)?;
        let pool_size = std::mem::size_of::<Pool>();
        let pool_lamports = rent.minimum_balance(pool_size);
        
        invoke_signed(
            &system_instruction::create_account(
                payer_info.key,
                &pool_pda,
                pool_lamports,
                pool_size as u64,
                program_id,
            ),
            &[payer_info.clone(), pool_info.clone(), system_program_info.clone()],
            &[&[b"privacy_pool", &[pool_bump]]],
        )?;
        
        let pool = Pool {
            is_initialized: true,
            merkle_tree_height,
            total_amount: 0,
        };
        
        pool.serialize(&mut *pool_info.data.borrow_mut())?;
        
        let merkle_tree_size = std::mem::size_of::<MerkleTree>();
        let merkle_tree_lamports = rent.minimum_balance(merkle_tree_size);
        
        invoke_signed(
            &system_instruction::create_account(
                payer_info.key,
                &merkle_tree_pda,
                merkle_tree_lamports,
                merkle_tree_size as u64,
                program_id,
            ),
            &[payer_info.clone(), merkle_tree_info.clone(), system_program_info.clone()],
            &[&[b"merkle_tree", &[merkle_tree_bump]]],
        )?;
        
        let merkle_tree = MerkleTree::new(merkle_tree_height);
        merkle_tree.serialize(&mut *merkle_tree_info.data.borrow_mut())?;
        
        Ok(())
    }
    
    fn process_shield(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        commitment: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        
        let payer_info = next_account_info(account_info_iter)?;
        let pool_info = next_account_info(account_info_iter)?;
        let merkle_tree_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;
        
        let (pool_pda, _) = find_pool_pda(program_id);
        if pool_pda != *pool_info.key {
            return Err(PrivacyError::InvalidPool.into());
        }
        
        let (merkle_tree_pda, _) = find_merkle_tree_pda(program_id);
        if merkle_tree_pda != *merkle_tree_info.key {
            return Err(ProgramError::InvalidAccountData);
        }
        
        invoke(
            &system_instruction::transfer(payer_info.key, &pool_pda, amount),
            &[payer_info.clone(), pool_info.clone(), system_program_info.clone()],
        )?;
        
        let mut pool = Pool::try_from_slice(&pool_info.data.borrow())?;
        pool.total_amount += amount;
        pool.serialize(&mut *pool_info.data.borrow_mut())?;
        
        let mut merkle_tree = MerkleTree::try_from_slice(&merkle_tree_info.data.borrow())?;
        merkle_tree.insert(&commitment)?;
        merkle_tree.serialize(&mut *merkle_tree_info.data.borrow_mut())?;
        
        Ok(())
    }
    
    fn process_withdraw(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        root: [u8; 32],
        nullifier_hash: [u8; 32],
        recipient: [u8; 32],
        proof: Vec<u8>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        
        let payer_info = next_account_info(account_info_iter)?;
        let pool_info = next_account_info(account_info_iter)?;
        let merkle_tree_info = next_account_info(account_info_iter)?;
        let nullifier_info = next_account_info(account_info_iter)?;
        let recipient_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;
        
        let (pool_pda, pool_bump) = find_pool_pda(program_id);
        if pool_pda != *pool_info.key {
            return Err(PrivacyError::InvalidPool.into());
        }
        
        let (merkle_tree_pda, _) = find_merkle_tree_pda(program_id);
        if merkle_tree_pda != *merkle_tree_info.key {
            return Err(ProgramError::InvalidAccountData);
        }
        
        let (nullifier_pda, nullifier_bump) = find_nullifier_pda(program_id, &nullifier_hash);
        if nullifier_pda != *nullifier_info.key {
            return Err(ProgramError::InvalidAccountData);
        }
        
        let recipient_pubkey = Utils::bytes_to_pubkey(&recipient);
        if recipient_pubkey != *recipient_info.key {
            return Err(PrivacyError::InvalidRecipient.into());
        }
        
        let merkle_tree = MerkleTree::try_from_slice(&merkle_tree_info.data.borrow())?;
        if merkle_tree.root != root {
            return Err(PrivacyError::InvalidRoot.into());
        }
        
        if !nullifier_info.data_is_empty() {
            return Err(PrivacyError::NullifierAlreadyUsed.into());
        }
        
        // In production, this would use the actual verification key and public inputs
        let verification_key = &[0u8; 32]; // Placeholder
        let public_inputs = &[0u8; 32]; // Placeholder
        
        let is_valid = Verifier::verify_withdrawal_proof(
            &proof,
            public_inputs,
            verification_key,
        )?;
        
        if !is_valid {
            return Err(PrivacyError::InvalidProof.into());
        }
        
        let rent = Rent::from_account_info(rent_info)?;
        let nullifier_size = std::mem::size_of::<Nullifier>();
        let nullifier_lamports = rent.minimum_balance(nullifier_size);
        
        invoke_signed(
            &system_instruction::create_account(
                payer_info.key,
                &nullifier_pda,
                nullifier_lamports,
                nullifier_size as u64,
                program_id,
            ),
            &[payer_info.clone(), nullifier_info.clone(), system_program_info.clone()],
            &[&[b"nullifier", &nullifier_hash, &[nullifier_bump]]],
        )?;
        
        let nullifier = Nullifier {
            is_initialized: true,
            nullifier_hash,
        };
        
        nullifier.serialize(&mut *nullifier_info.data.borrow_mut())?;
        
        let mut pool = Pool::try_from_slice(&pool_info.data.borrow())?;
        
        if pool.total_amount < amount {
            return Err(PrivacyError::InsufficientFunds.into());
        }
        
        pool.total_amount -= amount;
        pool.serialize(&mut *pool_info.data.borrow_mut())?;
        
        invoke_signed(
            &system_instruction::transfer(&pool_pda, &recipient_pubkey, amount),
            &[pool_info.clone(), recipient_info.clone(), system_program_info.clone()],
            &[&[b"privacy_pool", &[pool_bump]]],
        )?;
        
        Ok(())
    }
}
