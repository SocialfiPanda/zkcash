use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::str::FromStr;

/// Test keypair for signing transactions
pub const TEST_KEYPAIR_BYTES: [u8; 64] = [
    157, 12, 63, 94, 109, 133, 95, 152, 240, 33, 168, 95, 177, 253, 20, 215,
    87, 135, 23, 10, 211, 131, 109, 26, 160, 27, 33, 131, 31, 85, 185, 17,
    179, 242, 216, 130, 183, 154, 0, 248, 193, 153, 111, 111, 129, 146, 128, 157,
    61, 152, 22, 147, 11, 130, 213, 186, 194, 0, 129, 142, 54, 111, 116, 75,
];

/// Default merkle tree height for tests
pub const MERKLE_TREE_HEIGHT: u8 = 20;

/// Mock commitment for shield tests
pub const MOCK_COMMITMENT: [u8; 32] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
];

/// Mock root for withdraw tests
pub const MOCK_ROOT: [u8; 32] = [
    32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17,
    16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
];

/// Mock nullifier hash for withdraw tests
pub const MOCK_NULLIFIER_HASH: [u8; 32] = [
    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160,
    170, 180, 190, 200, 210, 220, 230, 240, 250, 240, 230, 220, 210, 200, 190, 180,
];

/// Mock recipient for withdraw tests
pub const MOCK_RECIPIENT: Pubkey = Pubkey::new_from_array([
    100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250,
    250, 240, 230, 220, 210, 200, 190, 180, 170, 160, 150, 140, 130, 120, 110, 100,
]);

/// Mock proof for withdraw tests (256 bytes)
pub const MOCK_PROOF: [u8; 256] = [
    // Proof A (64 bytes)
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
    
    // Proof B (128 bytes)
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128,
    129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144,
    145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
    161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
    177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    
    // Proof C (64 bytes)
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224,
    225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240,
    241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 0,
];

/// Get the test keypair
pub fn get_test_keypair() -> Keypair {
    Keypair::from_bytes(&TEST_KEYPAIR_BYTES).unwrap()
}

/// Get the program ID
pub fn get_program_id() -> Pubkey {
    Pubkey::from_str("ZKCashProgramPubkey11111111111111111111111").unwrap()
}

/// Get mock proof as vector
pub fn get_mock_proof() -> Vec<u8> {
    MOCK_PROOF.to_vec()
}

/// Generate a sample merkle path for testing
pub fn generate_sample_merkle_path(depth: usize) -> (Vec<[u8; 32]>, Vec<u8>) {
    let mut path = Vec::with_capacity(depth);
    let mut indices = Vec::with_capacity((depth + 7) / 8);
    
    // Fill indices with bytes where each bit represents a direction
    // For simplicity, we'll use alternating left-right pattern
    for i in 0..((depth + 7) / 8) {
        indices.push(0b10101010); // Alternating 1s and 0s
    }
    
    // Generate path elements
    for i in 0..depth {
        let mut element = [0u8; 32];
        // Fill with some deterministic pattern based on index
        for j in 0..32 {
            element[j] = ((i * j) % 256) as u8;
        }
        path.push(element);
    }
    
    (path, indices)
}
