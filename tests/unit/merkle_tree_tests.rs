use crate::common::fixtures::MOCK_COMMITMENT;
use zkcash::state::MerkleTree;
use solana_program::program_error::ProgramError;

#[cfg(test)]
mod tests {
    use super::*;
    
    const MERKLE_TREE_HEIGHT: u8 = 10;
    
    /// Test creating a new, empty merkle tree
    #[test]
    fn test_new_merkle_tree() {
        // Create a new merkle tree
        let merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        
        // Verify initial state
        assert!(merkle_tree.is_initialized);
        assert_eq!(merkle_tree.height, MERKLE_TREE_HEIGHT);
        assert_eq!(merkle_tree.current_index, 0);
        assert_eq!(merkle_tree.root, [0u8; 32]);
        assert_eq!(merkle_tree.filled_subtrees.len(), MERKLE_TREE_HEIGHT as usize);
        
        // All filled subtrees should be zero
        for subtree in merkle_tree.filled_subtrees.iter() {
            assert_eq!(*subtree, [0u8; 32]);
        }
    }
    
    /// Test inserting a leaf into a merkle tree
    #[test]
    fn test_insert_leaf() {
        // Create a new merkle tree
        let mut merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        
        // Initial state
        assert_eq!(merkle_tree.current_index, 0);
        let initial_root = merkle_tree.root;
        
        // Insert a leaf
        let leaf = MOCK_COMMITMENT;
        merkle_tree.insert(&leaf).unwrap();
        
        // Verify the state after insertion
        assert_eq!(merkle_tree.current_index, 1);
        assert_ne!(merkle_tree.root, initial_root);
        
        // First filled subtree should match the leaf
        assert_eq!(merkle_tree.filled_subtrees[0], leaf);
    }
    
    /// Test inserting multiple leaves into a merkle tree
    #[test]
    fn test_insert_multiple_leaves() {
        // Create a new merkle tree with a small height for testing
        let height = 4;
        let mut merkle_tree = MerkleTree::new(height);
        
        // Insert several leaves
        let leaf1 = MOCK_COMMITMENT;
        let mut leaf2 = MOCK_COMMITMENT;
        leaf2[0] = 1;
        let mut leaf3 = MOCK_COMMITMENT;
        leaf3[0] = 2;
        
        merkle_tree.insert(&leaf1).unwrap();
        let root1 = merkle_tree.root;
        
        merkle_tree.insert(&leaf2).unwrap();
        let root2 = merkle_tree.root;
        
        merkle_tree.insert(&leaf3).unwrap();
        let root3 = merkle_tree.root;
        
        // Verify the state after each insertion
        assert_eq!(merkle_tree.current_index, 3);
        assert_ne!(root1, root2);
        assert_ne!(root2, root3);
    }
    
    /// Test inserting the maximum number of leaves
    #[test]
    fn test_insert_max_leaves() {
        // Create a new merkle tree with a small height for testing
        let mut merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        
        // Calculate max number of leaves (2^height)
        let max_leaves = 1 << MERKLE_TREE_HEIGHT;
        
        // Insert leaves up to max - 1
        for i in 0..max_leaves {
            let mut leaf = [0u8; 32];
            leaf[0] = (i % 255) as u8;
            leaf[1] = ((i / 255) % 255) as u8;
            
            assert!(merkle_tree.insert(&leaf).is_ok());
            assert_eq!(merkle_tree.current_index, i + 1);
        }
        
        // Insert one more leaf, should fail
        let result = merkle_tree.insert(&[42u8; 32]);
        assert!(result.is_err());
    }
    
    /// Test deserializing and initializing a merkle tree
    #[test]
    fn test_merkle_tree_serialization() {
        // Create a merkle tree
        let merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        
        // Serialize and deserialize
        let serialized = borsh::to_vec(&merkle_tree).unwrap();
        let deserialized: MerkleTree = borsh::from_slice(&serialized).unwrap();
        
        // Test an uninitialized tree
        let mut uninitialized_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        uninitialized_tree.is_initialized = false;
        
        // Check the is_initialized method
        assert!(merkle_tree.is_initialized());
        assert!(deserialized.is_initialized());
        assert!(!uninitialized_tree.is_initialized());
    }
    
    /// Test merkle tree capacity limits
    #[test]
    fn test_merkle_tree_capacity() {
        // Create a small merkle tree for testing capacity
        let height = 3;
        let mut merkle_tree = MerkleTree::new(height);
        
        // Calculate the capacity
        let capacity = 1 << height;
        
        // Insert leaves up to capacity
        for i in 0..capacity {
            let mut leaf = [0u8; 32];
            leaf[0] = i as u8;
            let result = merkle_tree.insert(&leaf);
            assert!(result.is_ok());
        }
        
        // Verify the tree is full
        assert_eq!(merkle_tree.current_index as usize, capacity);
        
        // Try to insert one more leaf
        let extra_leaf = [255u8; 32];
        let result = merkle_tree.insert(&extra_leaf);
        
        // Verify the insertion fails
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ProgramError::InvalidArgument);
    }
    
    /// Test tree state after multiple insertions
    #[test]
    fn test_merkle_tree_state_after_inserts() {
        // Create a new merkle tree
        let mut merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        
        // Insert multiple leaves
        let num_leaves = 5;
        let mut previous_roots = Vec::with_capacity(num_leaves);
        
        for i in 0..num_leaves {
            // Save the current root
            previous_roots.push(merkle_tree.root);
            
            // Create a leaf with a unique value
            let mut leaf = [0u8; 32];
            leaf[0] = i as u8;
            
            // Insert the leaf
            let result = merkle_tree.insert(&leaf);
            assert!(result.is_ok());
            
            // Verify the current index was incremented
            assert_eq!(merkle_tree.current_index as usize, i + 1);
            
            // Verify the root changed
            assert_ne!(merkle_tree.root, previous_roots[i]);
        }
        
        // Verify all roots were different
        for i in 0..previous_roots.len() - 1 {
            assert_ne!(previous_roots[i], previous_roots[i + 1]);
        }
    }
    
    /// Test IsInitialized trait implementation
    #[test]
    fn test_is_initialized() {
        use solana_program::program_pack::IsInitialized;
        
        // Create a new merkle tree
        let merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        
        // Verify it's initialized
        assert!(merkle_tree.is_initialized());
        
        // Create an uninitialized merkle tree
        let mut uninitialized_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        uninitialized_tree.is_initialized = false;
        
        // Verify it's not initialized
        assert!(!uninitialized_tree.is_initialized());
    }
}
