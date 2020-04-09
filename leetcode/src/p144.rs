// https://leetcode-cn.com/problems/binary-tree-preorder-traversal/

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

// root -> left -> right
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    if let Some(root_node) = root.as_ref() {
        traversal(root_node, &mut res);
    }
    res
}

fn traversal(root_node: &Rc<RefCell<TreeNode>>, res: &mut Vec<i32>) {
    res.push(root_node.borrow().val);
    if let Some(left_node) = root_node.borrow().left.as_ref() {
        traversal(left_node, res);
    }
    if let Some(right_node) = root_node.borrow().right.as_ref() {
        traversal(right_node, res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1
    ///   2
    /// 3
    #[test]
    fn test_p144() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
        n2.borrow_mut().left = Some(n3);
        n1.borrow_mut().right = Some(n2);

        assert_eq!(vec![1, 2, 3], preorder_traversal(Some(n1)));
    }
}
