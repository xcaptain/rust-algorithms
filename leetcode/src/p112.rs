// https://leetcode-cn.com/problems/path-sum/

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

/// dfs + recursive
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    let mut res = vec![];
    construct_path(root, vec![], &mut res);
    for path in res {
        if path.iter().sum::<i32>() == sum {
            return true;
        }
    }
    false
}

fn construct_path(
    root: Option<Rc<RefCell<TreeNode>>>,
    mut path: Vec<i32>,
    paths: &mut Vec<Vec<i32>>,
) {
    if let Some(cur_node) = root {
        let ref_node = cur_node.as_ref().borrow();
        path.push(ref_node.val);
        if ref_node.left.is_none() && ref_node.right.is_none() {
            paths.push(path);
        } else {
            construct_path(ref_node.left.clone(), path.clone(), paths);
            construct_path(ref_node.right.clone(), path, paths);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p112() {
        let n7 = TreeNode::new(7);
        let n2 = TreeNode::new(2);
        let mut n11 = TreeNode::new(11);
        n11.left = Some(Rc::new(RefCell::new(n7)));
        n11.right = Some(Rc::new(RefCell::new(n2)));

        let n1 = TreeNode::new(1);
        let mut n4 = TreeNode::new(4);
        n4.right = Some(Rc::new(RefCell::new(n1)));

        let n13 = TreeNode::new(13);
        let mut n8 = TreeNode::new(8);
        n8.left = Some(Rc::new(RefCell::new(n13)));
        n8.right = Some(Rc::new(RefCell::new(n4)));

        let mut n4_1 = TreeNode::new(4);
        n4_1.left = Some(Rc::new(RefCell::new(n11)));

        let mut n5 = TreeNode::new(5);
        n5.left = Some(Rc::new(RefCell::new(n4_1)));
        n5.right = Some(Rc::new(RefCell::new(n8)));

        assert!(has_path_sum(Some(Rc::new(RefCell::new(n5))), 22));
    }
}
