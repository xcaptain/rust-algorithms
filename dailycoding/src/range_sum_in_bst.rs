//! This problem was asked by Google.
//! Given a binary search tree and a range [a, b] (inclusive), return the sum of the
//! elements of the binary search tree within the range.
//!
//! For example, given the following tree:
//!     5
//!    / \
//!   3   8
//!  / \ / \
//! 2  4 6  10
//! and the range [4, 9], return 23 (5 + 4 + 6 + 8).

use data_structures::tree::binary_tree::Tree;

pub fn get_range_sum_in_bst(t: &Tree<i32>, rg: [i32; 2]) -> i32 {
    let arr = t.inorder_traverse();
    let [start, end] = rg;

    let mut s = 0;

    for num in arr {
        if num >= start && num <= end {
            s += num;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_range_sum_in_bst() {
        let tree1 = Tree::from_vec(vec![5, 3, 8, 2, 4, 6, 10]);

        assert_eq!(23, get_range_sum_in_bst(&tree1, [4, 9]));
    }
}
