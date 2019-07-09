// https://leetcode-cn.com/problems/same-tree/

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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut p_arr: Vec<Option<i32>> = vec![];
    let mut q_arr: Vec<Option<i32>> = vec![];
    traverse_val(p, &mut p_arr);
    traverse_val(q, &mut q_arr);
    if p_arr.len() != q_arr.len() {
        return false;
    }
    for i in 0..p_arr.len() {
        if p_arr[i] != q_arr[i] {
            return false;
        }
    }
    return true;
}

fn traverse_val(t: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<Option<i32>>) {
    if let Some(cur_node) = t {
        let ref_node = Rc::try_unwrap(cur_node).unwrap();
        let tree_node = ref_node.into_inner();
        arr.push(Some(tree_node.val));
        traverse_val(tree_node.left, arr);
        traverse_val(tree_node.right, arr);
    // if you want to do real infix traversal, uncomment these lines
    // if !tree_node.left.is_none() {
    //     traverse_val(tree_node.left, arr);
    // }
    // if !tree_node.right.is_none() {
    //     traverse_val(tree_node.right, arr);
    // }
    } else {
        arr.push(None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        let t2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));

        let t3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let t4 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));

        assert_eq!(false, is_same_tree(t1, t2));
        assert_eq!(false, is_same_tree(t3, t4));
    }
}
