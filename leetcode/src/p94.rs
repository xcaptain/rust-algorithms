// https://leetcode-cn.com/problems/binary-tree-inorder-traversal/

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
use std::rc::Rc;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    traversal(root, &mut res);
    res
}

fn traversal(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(cur_node) = root {
        let ref_node = Rc::try_unwrap(cur_node).unwrap();
        let tree_node = ref_node.into_inner();
        if !tree_node.left.is_none() {
            traversal(tree_node.left, res);
        }
        res.push(tree_node.val);
        if !tree_node.right.is_none() {
            traversal(tree_node.right, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p94() {
        let mut n1 = TreeNode::new(1);
        let mut n2 = TreeNode::new(2);
        let n3 = TreeNode::new(3);
        n2.left = Some(Rc::new(RefCell::new(n3)));
        n1.right = Some(Rc::new(RefCell::new(n2)));

        let arr1 = inorder_traversal(Some(Rc::new(RefCell::new(n1))));
        assert_eq!(vec![1, 3, 2], arr1);
    }
}
