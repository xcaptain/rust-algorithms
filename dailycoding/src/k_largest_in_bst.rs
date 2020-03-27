//! This problem was asked by Dropbox.
//! Given the root to a binary search tree, find the second largest node in the tree.
//!
//! Hint:
//! inorder traverse the binary tree we can have a incremental array

use data_structures::tree::binary_tree::Tree;

pub fn k_largest_in_bst(t: &Tree<i32>, k: usize) -> i32 {
    t.k_largest(k).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_largest_in_bst() {
        let tree1 = Tree::from_vec(vec![2, 1, 3, 2, 4]);

        assert_eq!(4, k_largest_in_bst(&tree1, 1));
        assert_eq!(3, k_largest_in_bst(&tree1, 2));
    }
}
