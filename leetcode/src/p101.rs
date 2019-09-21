// https://leetcode-cn.com/problems/symmetric-tree/

use std::cell::RefCell;
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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(cur_node) => {
            let ref_node = Rc::try_unwrap(cur_node).unwrap();
            let tree_node = ref_node.into_inner();
            are_symmetric(tree_node.left, tree_node.right)
        }
    }
}

// test 2 sub tree are symmetric
fn are_symmetric(
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if left.is_none() {
        return right.is_none();
    }
    if right.is_none() {
        return left.is_none();
    }

    let left_ref_node = Rc::try_unwrap(left.unwrap()).unwrap();
    let left_tree_node = left_ref_node.into_inner();
    let right_ref_node = Rc::try_unwrap(right.unwrap()).unwrap();
    let right_tree_node = right_ref_node.into_inner();

    left_tree_node.val == right_tree_node.val
        && are_symmetric(left_tree_node.left, right_tree_node.right)
        && are_symmetric(left_tree_node.right, right_tree_node.left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symmetric() {
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

        let t2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(true, is_symmetric(t1));
        assert_eq!(false, is_symmetric(t2));
    }
}
