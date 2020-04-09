// https://leetcode-cn.com/problems/sum-root-to-leaf-numbers/

use std::cell::RefCell;
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

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = vec![];
    dfs(root.as_ref(), &mut res, vec![]);

    let mut ans = 0;
    for row in res {
        let t: String = row
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("");
        ans += t.parse::<i32>().unwrap();
    }
    ans
}

fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>, mut cur: Vec<i32>) {
    if let Some(root_node) = root {
        cur.push(root_node.borrow().val);
        if root_node.borrow().left.is_none() && root_node.borrow().right.is_none() {
            res.push(cur);
            return;
        } else {
            dfs(root_node.borrow().left.as_ref(), res, cur.clone());
            dfs(root_node.borrow().right.as_ref(), res, cur);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p129() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
        n1.borrow_mut().left = Some(n2);
        n1.borrow_mut().right = Some(n3);

        assert_eq!(25, sum_numbers(Some(n1)));

        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(5)));
        n1.borrow_mut().right = Some(n2);

        assert_eq!(15, sum_numbers(Some(n1)));
    }
}
