//! find the boottom view of a binary tree
//! This problem was asked by Yelp.
//! The horizontal distance of a binary tree node describes how far left or right the node will be when the tree is printed out.
//! More rigorously, we can define it as follows:
//!     The horizontal distance of the root is 0.
//!     The horizontal distance of a left child is hd(parent) - 1.
//!     The horizontal distance of a right child is hd(parent) + 1.
//!
//! For example, for the following tree, hd(1) = -2, and hd(6) = 0.
//!              5
//!           /     \
//!         3         7
//!       /  \      /   \
//!     1     4    6     9
//!    /                /
//!   0                8
//! The bottom view of a tree, then, consists of the lowest node at each horizontal distance. If there are two nodes at the same depth and horizontal distance, either is acceptable.
//! For this tree, for example, the bottom view could be [0, 1, 3, 6, 8, 9].
//! Given the root to a binary tree, return its bottom view.

use data_structures::tree::binary_tree::Tree;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn get_bottom_view(t: Tree<i32>) -> Vec<i32> {
    let res = bfs_traverse(t);

    // println!("{:?}", res);
    let hd_min: i32 = res.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let hd_max: i32 = res.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    // println!("{}, {}", hd_max, hd_min);

    let mut ans = vec![];
    for i in hd_min..=hd_max {
        let row_i = res
            .iter()
            .filter(|e| e.1 == i)
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .2;
        ans.push(row_i);
    }
    ans
}

/// depth, hd, value
fn bfs_traverse(t: Tree<i32>) -> Vec<(usize, i32, i32)> {
    let mut res = vec![];

    let mut q = VecDeque::new();
    if let Some(root_node) = t.root.as_ref() {
        q.push_back((Rc::clone(root_node), 0));
    }

    let mut depth = 0;
    while !q.is_empty() {
        let l = q.len();
        for _i in 0..l {
            let (cur_node, hd) = q.pop_front().unwrap();
            res.push((depth, hd, cur_node.borrow().elem));

            // if the current node has children, then push these children into
            // the queue, so we can continue traverse down the tree.
            // note: using map here is simpler than using match
            let borrowed_cur_node = cur_node.borrow();
            if let Some(v) = borrowed_cur_node.left.as_ref() {
                q.push_back((Rc::clone(v), hd - 1));
            }
            if let Some(v) = borrowed_cur_node.right.as_ref() {
                q.push_back((Rc::clone(v), hd + 1));
            }
        }

        depth += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use data_structures::tree::binary_tree::Tree;

    #[test]
    fn test_horizontal_view() {
        let tree1 = Tree::from_vec(vec![5, 3, 7, 1, 4, 0, 6, 9, 8]);
        assert_eq!(vec![0, 1, 3, 6, 8, 9], get_bottom_view(tree1));

        let tree2 = Tree::from_vec(vec![5, 3, 1, 0, 8, 7]);
        assert_eq!(vec![0, 1, 3, 7, 8], get_bottom_view(tree2));
    }
}
