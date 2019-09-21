// https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    depth(root)
}

fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(cur_node) => {
            let ref_node = Rc::try_unwrap(cur_node).unwrap();
            let tree_node = ref_node.into_inner();
            let left = tree_node.left;
            let right = tree_node.right;

            if left.is_none() {
                return max(1, depth(right) + 1);
            }
            if right.is_none() {
                return max(1, depth(left) + 1);
            }
            let left_depth = depth(left) + 1;
            let right_depth = depth(right) + 1;

            max(left_depth, right_depth)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(3, max_depth(t1));
    }
}
