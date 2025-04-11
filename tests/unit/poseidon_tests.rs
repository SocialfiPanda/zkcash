use crate::common::fixtures::{MOCK_COMMITMENT, generate_sample_merkle_path};
use zkcash::poseidon;

#[cfg(test)]
mod tests {
    use super::*;
    use light_poseidon::{Poseidon, PoseidonBytesHasher};
    use ark_bn254::Fr;

    /// Test the hash_1 function
    #[test]
    fn test_hash_1() {
        // Create input
        let input = MOCK_COMMITMENT;
        
        // Call the function
        let result = poseidon::hash_1(&input).unwrap();
        
        // Verify the result is not all zeros
        assert_ne!(result, [0u8; 32]);
        
        // Verify the result matches the expected output from light-poseidon
        let mut poseidon = Poseidon::<Fr>::new_circom(1).unwrap();
        let expected = poseidon.hash_bytes_be(&[&input]).unwrap();
        assert_eq!(result, expected);
    }
    
    /// Test the hash_2 function
    #[test]
    fn test_hash_2() {
        // Create inputs
        let left = MOCK_COMMITMENT;
        let right = [42u8; 32];
        
        // Call the function
        let result = poseidon::hash_2(&left, &right).unwrap();
        
        // Verify the result is not all zeros
        assert_ne!(result, [0u8; 32]);
        
        // Verify the result matches the expected output from light-poseidon
        let mut poseidon = Poseidon::<Fr>::new_circom(2).unwrap();
        let expected = poseidon.hash_bytes_be(&[&left, &right]).unwrap();
        assert_eq!(result, expected);
    }
    
    /// Test the hash_left_right function (alias for hash_2)
    #[test]
    fn test_hash_left_right() {
        // Create inputs
        let left = MOCK_COMMITMENT;
        let right = [42u8; 32];
        
        // Call both functions
        let result1 = poseidon::hash_2(&left, &right).unwrap();
        let result2 = poseidon::hash_left_right(&left, &right).unwrap();
        
        // Verify they produce the same result
        assert_eq!(result1, result2);
    }
    
    /// Test the compute_merkle_root function
    #[test]
    fn test_compute_merkle_root() {
        // Create a sample merkle path
        let depth = 10;
        let (path, indices) = generate_sample_merkle_path(depth);
        let leaf = MOCK_COMMITMENT;
        
        // Call the function
        let result = poseidon::compute_merkle_root(&leaf, &path, &indices).unwrap();
        
        // Verify the result is not all zeros
        assert_ne!(result, [0u8; 32]);
        
        // Manually compute the root to verify
        let mut current = leaf;
        for i in 0..path.len() {
            let path_element = path[i];
            let index_bit = indices[i / 8] & (1 << (i % 8));
            
            if index_bit == 0 {
                // Current is left, path_element is right
                current = poseidon::hash_left_right(&current, &path_element).unwrap();
            } else {
                // Current is right, path_element is left
                current = poseidon::hash_left_right(&path_element, &current).unwrap();
            }
        }
        
        // Verify the manual computation matches the function result
        assert_eq!(result, current);
    }
    
    /// Test error handling in compute_merkle_root
    #[test]
    fn test_compute_merkle_root_error() {
        // Create an empty path
        let path: Vec<[u8; 32]> = Vec::new();
        let indices: Vec<u8> = Vec::new();
        let leaf = MOCK_COMMITMENT;
        
        // Call the function with invalid inputs
        let result = poseidon::compute_merkle_root(&leaf, &path, &indices);
        
        // Verify it returns an error
        assert!(result.is_err());
    }
    
    /// Test compatibility with light-poseidon library
    #[test]
    fn test_poseidon_compatibility() {
        // Create inputs
        let input1 = MOCK_COMMITMENT;
        let input2 = [42u8; 32];
        
        // Use zkcash implementation
        let result1 = poseidon::hash_1(&input1).unwrap();
        let result2 = poseidon::hash_2(&input1, &input2).unwrap();
        
        // Use light-poseidon directly
        let mut poseidon1 = Poseidon::<Fr>::new_circom(1).unwrap();
        let mut poseidon2 = Poseidon::<Fr>::new_circom(2).unwrap();
        let expected1 = poseidon1.hash_bytes_be(&[&input1]).unwrap();
        let expected2 = poseidon2.hash_bytes_be(&[&input1, &input2]).unwrap();
        
        // Verify results match
        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
    }
}
