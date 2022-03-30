// https://leetcode-cn.com/problems/binary-tree-zigzag-level-order-traversal/

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

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut q = VecDeque::new();
    let mut res = vec![];
    let mut is_right_to_left = false; // true: 右->左， false： 左->右
    if let Some(root_node) = root.as_ref() {
        q.push_back(Rc::clone(root_node));

        while !q.is_empty() {
            let l = q.len();
            let mut t: Vec<i32> = vec![]; // 临时变量，包含本层的结点的值
            for _i in 0..l {
                let node = q.pop_front().unwrap();
                t.push(node.borrow().val);

                let borrowed_node = node.borrow();
                if let Some(v) = borrowed_node.left.as_ref() {
                    q.push_back(Rc::clone(v));
                }
                if let Some(v) = borrowed_node.right.as_ref() {
                    q.push_back(Rc::clone(v));
                }
            }

            if !is_right_to_left {
                res.push(t);
            } else {
                res.push(t.into_iter().rev().collect::<Vec<i32>>());
            }

            is_right_to_left = !is_right_to_left;
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
    fn test_p103() {
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
            vec![vec![3], vec![20, 9], vec![15, 7]],
            zigzag_level_order(Some(n1))
        );
    }

    ///     1
    ///   2   3
    /// 4       5
    #[test]
    fn test_p103_1() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let n5 = Rc::new(RefCell::new(TreeNode::new(5)));

        n2.borrow_mut().left = Some(n4);
        n3.borrow_mut().right = Some(n5);

        n1.borrow_mut().left = Some(n2);
        n1.borrow_mut().right = Some(n3);

        assert_eq!(
            vec![vec![1], vec![3, 2], vec![4, 5]],
            zigzag_level_order(Some(n1))
        );
    }
}
