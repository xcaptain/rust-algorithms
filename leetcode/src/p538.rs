// https://leetcode-cn.com/problems/convert-bst-to-greater-tree/

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

pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = root;
    traverse(&mut root, &mut 0);
    root
}

fn traverse(root: &mut Option<Rc<RefCell<TreeNode>>>, s: &mut i32) {
    if let Some(root_node) = root {
        let mut right_node = root_node.as_ref().borrow().right.clone();
        traverse(&mut right_node, s);
        *s += root_node.borrow().val;
        root_node.borrow_mut().val = *s;
        let mut left_node = root_node.as_ref().borrow().left.clone();
        traverse(&mut left_node, s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p538() {
        let t2 = TreeNode::new(2);
        let t13 = TreeNode::new(13);
        let mut t5 = TreeNode::new(5);
        t5.left = Some(Rc::new(RefCell::new(t2)));
        t5.right = Some(Rc::new(RefCell::new(t13)));

        let t20 = TreeNode::new(20);
        let t13 = TreeNode::new(13);
        let mut t18 = TreeNode::new(18);
        t18.left = Some(Rc::new(RefCell::new(t20)));
        t18.right = Some(Rc::new(RefCell::new(t13)));

        assert_eq!(
            Some(Rc::new(RefCell::new(t18))),
            convert_bst(Some(Rc::new(RefCell::new(t5))))
        );
    }
}
