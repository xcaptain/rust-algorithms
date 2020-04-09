// https://leetcode-cn.com/problems/binary-tree-level-order-traversal/

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut q = VecDeque::new();
    let mut res = vec![];
    if let Some(root_node) = root.as_ref() {
        q.push_back(Rc::clone(root_node));

        while !q.is_empty() {
            let l = q.len();
            let mut t: Vec<i32> = vec![]; // 临时变量，包含本层的结点的值
            for _i in 0..l {
                let node = q.pop_front().unwrap();
                t.push(node.borrow().val);
                node.borrow().left.as_ref().map(|v| {
                    q.push_back(Rc::clone(v));
                });
                node.borrow().right.as_ref().map(|v| {
                    q.push_back(Rc::clone(v));
                });
            }
            res.push(t);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    ///   3
    /// 9   20
    ///   15   7
    #[test]
    fn test_p102() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(9)));
        let n3 = Rc::new(RefCell::new(TreeNode::new(20)));
        let n4 = Rc::new(RefCell::new(TreeNode::new(15)));
        let n5 = Rc::new(RefCell::new(TreeNode::new(7)));

        n3.borrow_mut().left = Some(n4);
        n3.borrow_mut().right = Some(n5);

        n1.borrow_mut().left = Some(n2);
        n1.borrow_mut().right = Some(n3);

        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7],],
            level_order(Some(n1))
        );
    }
}
