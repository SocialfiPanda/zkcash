use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum PrivacyInstruction {
    Initialize {
        merkle_tree_height: u8,
    },
    
    Shield {
        amount: u64,
        commitment: [u8; 32],
    },
    
    Withdraw {
        amount: u64,
        root: [u8; 32],
        nullifier_hash: [u8; 32],
        recipient: [u8; 32],
        proof: Vec<u8>,
    },
}
