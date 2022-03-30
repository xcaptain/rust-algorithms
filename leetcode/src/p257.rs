// https://leetcode-cn.com/problems/binary-tree-paths/

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
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    construct_path(root, String::new(), &mut paths);
    paths
}

fn construct_path(root: Option<Rc<RefCell<TreeNode>>>, mut path: String, paths: &mut Vec<String>) {
    if let Some(cur_node) = root {
        let ref_node = cur_node.as_ref().borrow();
        path += ref_node.val.to_string().as_str();
        if ref_node.left.is_none() && ref_node.right.is_none() {
            // if is leaf node, then save to current path
            paths.push(path);
        } else {
            // not leaf node, do recursive
            path += "->";
            construct_path(ref_node.left.clone(), path.clone(), paths);
            construct_path(ref_node.right.clone(), path, paths);
        }
    }
}

/// use bfs and iteration
pub fn binary_tree_paths_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut stack: Vec<(Rc<RefCell<TreeNode>>, String)> = vec![];
    let mut paths: Vec<String> = vec![];
    if let Some(cur_node) = root {
        let root_val = cur_node.borrow().val;
        stack.push((cur_node, root_val.to_string()));
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            let s = node.0;
            let borrowed_node = s.as_ref().borrow();
            let path = node.1;

            if borrowed_node.left.is_none() && borrowed_node.right.is_none() {
                paths.push(path.clone());
            }
            if let Some(ref lft) = borrowed_node.left {
                stack.push((
                    Rc::clone(lft),
                    path.clone() + "->" + lft.borrow().val.to_string().as_str(),
                ));
            }
            if let Some(ref rht) = borrowed_node.right {
                stack.push((
                    Rc::clone(rht),
                    path + "->" + rht.borrow().val.to_string().as_str(),
                ));
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p257() {
        //    1
        //  /   \
        // 2     3
        // \
        //  5
        let t5 = TreeNode::new(5);
        let mut t2 = TreeNode::new(2);
        t2.left = Some(Rc::new(RefCell::new(t5)));
        let t3 = TreeNode::new(3);
        let mut t1 = TreeNode::new(1);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let tree = Some(Rc::new(RefCell::new(t1)));
        assert_eq!(
            vec![String::from("1->2->5"), String::from("1->3")],
            binary_tree_paths(tree)
        );
    }
}
