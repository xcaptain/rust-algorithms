// https://leetcode-cn.com/problems/binary-tree-level-order-traversal-ii/

// actually I don't quite understand this solution, it's just migration of a go solition

// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    if root.is_none() {
        return res;
    }
    let mut queue: VecDeque<TreeNode> = VecDeque::new();
    let root_node_rc = root.unwrap();
    let root_node_ref = Rc::try_unwrap(root_node_rc).unwrap();
    let root_node = root_node_ref.into_inner();
    queue.push_back(root_node);
    while !queue.is_empty() {
        let size = queue.len();
        let mut temp: Vec<i32> = vec![];
        for _i in 0..size {
            let node = queue.pop_front().unwrap();
            temp.push(node.val);
            if !node.left.is_none() {
                let left_node_rc = node.left.unwrap();
                let left_node_ref = Rc::try_unwrap(left_node_rc).unwrap();
                let left_node = left_node_ref.into_inner();
                queue.push_back(left_node);
            }
            if !node.right.is_none() {
                let right_node_rc = node.right.unwrap();
                let right_node_ref = Rc::try_unwrap(right_node_rc).unwrap();
                let right_node = right_node_ref.into_inner();
                queue.push_back(right_node);
            }
        }
        res.push(temp);
    }
    res.reverse();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order_bottom() {
        let arr = vec![vec![15, 7], vec![9, 20], vec![3]];
        let t1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(arr, level_order_bottom(t1));
    }
}
