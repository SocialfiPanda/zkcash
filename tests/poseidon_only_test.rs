use light_poseidon::{Poseidon, PoseidonBytesHasher};
use ark_bn254::Fr;
use zkcash::poseidon;

// Mock constant for testing
const MOCK_COMMITMENT: [u8; 32] = [42u8; 32];

#[test]
fn test_poseidon_hash() {
    // Test width 1
    let mut poseidon1 = Poseidon::<Fr>::new_circom(1).unwrap();
    let result1 = poseidon1.hash_bytes_be(&[&MOCK_COMMITMENT]).unwrap();
    
    // Verify not all zeros
    assert_ne!(result1, [0u8; 32]);
    
    // Test width 2
    let mut poseidon2 = Poseidon::<Fr>::new_circom(2).unwrap();
    let input2 = [1u8; 32];
    let result2 = poseidon2.hash_bytes_be(&[&MOCK_COMMITMENT, &input2]).unwrap();
    
    // Verify not all zeros
    assert_ne!(result2, [0u8; 32]);
    
    // Test consistency
    let mut poseidon3 = Poseidon::<Fr>::new_circom(1).unwrap();
    let result3 = poseidon3.hash_bytes_be(&[&MOCK_COMMITMENT]).unwrap();
    
    // Same inputs should give same outputs
    assert_eq!(result1, result3);
}

#[test]
fn test_our_poseidon_implementation() {
    // Test hash_1
    let result1 = poseidon::hash_1(&MOCK_COMMITMENT).unwrap();
    
    // Verify not all zeros
    assert_ne!(result1, [0u8; 32]);
    
    // Test hash_2
    let input2 = [1u8; 32];
    let result2 = poseidon::hash_2(&MOCK_COMMITMENT, &input2).unwrap();
    
    // Verify not all zeros
    assert_ne!(result2, [0u8; 32]);
    
    // Compare with direct implementation
    let mut direct_poseidon1 = Poseidon::<Fr>::new_circom(1).unwrap();
    let direct_result1 = direct_poseidon1.hash_bytes_be(&[&MOCK_COMMITMENT]).unwrap();
    
    assert_eq!(result1, direct_result1);
    
    // Compare hash_2 with direct implementation
    let mut direct_poseidon2 = Poseidon::<Fr>::new_circom(2).unwrap();
    let direct_result2 = direct_poseidon2.hash_bytes_be(&[&MOCK_COMMITMENT, &input2]).unwrap();
    
    assert_eq!(result2, direct_result2);
    
    // Test hash_left_right (should be alias for hash_2)
    let result3 = poseidon::hash_left_right(&MOCK_COMMITMENT, &input2).unwrap();
    assert_eq!(result3, result2);
}

#[test]
fn test_compute_merkle_root() {
    // Create leaf and path
    let leaf = MOCK_COMMITMENT;
    
    // Create a simple merkle path (3 levels)
    let path = vec![
        [1u8; 32],  // path element 1
        [2u8; 32],  // path element 2
        [3u8; 32],  // path element 3
    ];
    
    // Set path indices (binary path: 0, 1, 0)
    // In bytes: 0b00000010 (only bit 1 is set)
    let indices = vec![2u8]; // Second bit is 1, others are 0
    
    // Compute merkle root
    let root = poseidon::compute_merkle_root(&leaf, &path, &indices).unwrap();
    
    // Verify not all zeros
    assert_ne!(root, [0u8; 32]);
    
    // Manually compute the root
    let mut current = leaf;
    
    // Level 0: leaf is left, path[0] is right (index 0)
    current = poseidon::hash_left_right(&current, &path[0]).unwrap();
    
    // Level 1: path[1] is left, current is right (index 1)
    current = poseidon::hash_left_right(&path[1], &current).unwrap();
    
    // Level 2: current is left, path[2] is right (index 0)
    current = poseidon::hash_left_right(&current, &path[2]).unwrap();
    
    // Verify the manual computation matches the function result
    assert_eq!(root, current);
    
    // Test invalid input (empty path)
    let empty_path: Vec<[u8; 32]> = Vec::new();
    let empty_indices: Vec<u8> = Vec::new();
    let invalid_result = poseidon::compute_merkle_root(&leaf, &empty_path, &empty_indices);
    
    // Should return an error
    assert!(invalid_result.is_err());
} 