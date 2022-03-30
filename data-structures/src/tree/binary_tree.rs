use std::cell::RefCell;
use std::cmp::{Ord, Ordering};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode<T: Ord + Debug + Clone> {
    pub elem: T,
    pub left: Link<T>,
    pub right: Link<T>,
}

type Link<T> = Option<Rc<RefCell<TreeNode<T>>>>;

pub struct Tree<T: Ord + Debug + Clone> {
    pub root: Link<T>,
    pub total_nodes: i32,
}

impl<T: Ord + Debug + Clone> TreeNode<T> {
    pub fn new(t: T) -> Self {
        TreeNode {
            elem: t,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, t: T) {
        if self.elem > t {
            match self.left.take() {
                None => {
                    // println!("in insert left: {:?}", t);
                    self.left = Some(Rc::new(RefCell::new(TreeNode::new(t))))
                }
                Some(left_node) => {
                    left_node.borrow_mut().insert(t);
                    self.left = Some(left_node);
                }
            }
        } else {
            match self.right.take() {
                None => {
                    // println!("in insert right: {:?}", t);
                    self.right = Some(Rc::new(RefCell::new(TreeNode::new(t))));
                }
                Some(right_node) => {
                    right_node.borrow_mut().insert(t);
                    self.right = Some(right_node);
                }
            }
        }
    }

    pub fn find(&self, t: T) -> bool {
        match self.elem.cmp(&t) {
            Ordering::Equal => true,
            Ordering::Less => match self.right.as_ref() {
                None => false,
                Some(node) => node.borrow().find(t),
            },
            Ordering::Greater => match self.left.as_ref() {
                None => false,
                Some(node) => node.borrow().find(t),
            },
        }
    }
}

impl<T: Ord + Debug + Clone> Default for Tree<T> {
    fn default() -> Self {
        Tree::new()
    }
}

impl<T: Ord + Debug + Clone> Tree<T> {
    pub fn new() -> Self {
        Tree {
            root: None,
            total_nodes: 0,
        }
    }

    pub fn from_vec(arr: Vec<T>) -> Self {
        let mut t = Tree::new();
        for item in arr {
            t.insert(item);
        }
        t
    }

    pub fn insert(&mut self, t: T) {
        match self.root.as_ref() {
            None => {
                let node = TreeNode::new(t);
                self.root = Some(Rc::new(RefCell::new(node)));
                self.total_nodes += 1;
            }
            Some(root_node) => {
                root_node.borrow_mut().insert(t);
                self.total_nodes += 1;
            }
        }
    }

    /// 二叉树中序遍历，也就是 左 -> 中 -> 右
    ///
    /// TODO: 先序，后序遍历也是同样的思路，不过是否能用循环而不是递归呢?
    pub fn inorder_traverse(&self) -> Vec<T> {
        let mut res = vec![];
        Self::inorder_traverse_helper(self.root.as_ref(), &mut res);
        res
    }

    fn inorder_traverse_helper(root: Option<&Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        if let Some(cur_node) = root {
            let tree_node = cur_node.borrow_mut();
            if tree_node.left.is_some() {
                Self::inorder_traverse_helper(tree_node.left.as_ref(), res);
            }
            // println!("elem: {:?}", tree_node.elem);
            res.push(tree_node.elem.clone());
            if tree_node.right.is_some() {
                Self::inorder_traverse_helper(tree_node.right.as_ref(), res);
            }
        }
    }

    /// 层次遍历二叉树，基本思路是 BFS
    pub fn level_traverse(&self) -> Vec<T> {
        let mut res = vec![];

        let mut q = VecDeque::new();
        if let Some(root_node) = self.root.as_ref() {
            q.push_back(Rc::clone(root_node));
        }

        while !q.is_empty() {
            // iterate from 0 to q.len() if we need to know the depth of each node
            let cur_node = q.pop_front().unwrap();
            res.push(cur_node.borrow().elem.clone());

            // if the current node has children, then push these children into
            // the queue, so we can continue traverse down the tree.
            // note: using map here is simpler than using match
            let borrowed_cur_node = cur_node.borrow();
            if let Some(v) = borrowed_cur_node.left.as_ref() {
                q.push_back(Rc::clone(v));
            }
            if let Some(v) = borrowed_cur_node.right.as_ref() {
                q.push_back(Rc::clone(v));
            }
        }
        res
    }

    /// 深度遍历二叉树，也就是说如果某个结点有子结点则往下遍历
    ///     1
    ///       2
    ///         3
    ///       2   4
    ///  
    /// Examples:
    ///
    /// ```
    /// use data_structures::tree::binary_tree::Tree;
    /// let tree = Tree::from_vec(vec![1, 2, 3, 4, 2]);
    /// assert_eq!(
    ///     vec![vec![1, 2, 3, 2], vec![1, 2, 3, 4]],
    ///     tree.dfs_traverse()
    /// );
    /// ```
    pub fn dfs_traverse(&self) -> Vec<Vec<T>> {
        let mut res = vec![];
        Self::dfs_traverse_helper(self.root.as_ref(), vec![], &mut res);
        res
    }

    fn dfs_traverse_helper(
        root: Option<&Rc<RefCell<TreeNode<T>>>>,
        mut cur: Vec<T>,
        res: &mut Vec<Vec<T>>,
    ) {
        if let Some(root_node) = root {
            cur.push(root_node.borrow().elem.clone());
            if root_node.borrow().left.is_none() && root_node.borrow().right.is_none() {
                res.push(cur); // reached the end, stop recursive
            } else {
                Self::dfs_traverse_helper(root_node.borrow().left.as_ref(), cur.clone(), res);
                Self::dfs_traverse_helper(root_node.borrow().right.as_ref(), cur, res);
            }
        }
    }

    /// pick the root node of a tree, return a reference to the pointer
    pub fn peek(&self) -> Option<&Rc<RefCell<TreeNode<T>>>> {
        // self.root.as_ref().map(|node| node.borrow().elem.clone())
        self.root.as_ref()
    }

    /// find an element in the tree, if not exists then return false else true
    /// TODO: a better version would return Option<Rc<...>> so we can do some
    /// mutation in the future
    pub fn find(&self, t: T) -> bool {
        if let Some(cur_node) = self.root.as_ref() {
            return cur_node.borrow().find(t);
        }
        false
    }

    pub fn largest(&self) -> Option<T> {
        let arr = self.inorder_traverse();
        if arr.is_empty() {
            return None;
        }
        Some(arr.last().unwrap().clone())
    }

    /// find the kth largest element in the binary tree
    /// k starts from 1, which means when k == 1 it returns the largest
    pub fn k_largest(&self, k: usize) -> Option<T> {
        let arr = self.inorder_traverse();
        let l = arr.len();

        if l < k {
            return None;
        }
        Some(arr[l - k].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test insert a binary search tree
    ///        2
    ///      1   3
    ///        2
    #[test]
    fn test_insert() {
        let tree = Tree::from_vec(vec![2, 1, 3, 2]);
        assert_eq!(4, tree.total_nodes);
        let node1 = tree.root.as_ref().map(|node| node.borrow().elem);
        assert_eq!(Some(2), node1);

        let node2 = tree.root.as_ref().map(|node| {
            let left_node = &node.borrow().left;
            let elem2 = left_node.as_ref().map(|node2| node2.borrow().elem).unwrap();
            elem2
        });
        assert_eq!(Some(1), node2);
    }

    #[test]
    fn test_peek() {
        let tree1 = Tree::from_vec(vec![2, 1, 3, 2]);
        assert_eq!(Some(2), tree1.peek().map(|node| node.borrow().elem));

        // modify the root node element, before is 2 now is 3
        if let Some(node) = tree1.peek() {
            node.borrow_mut().elem = 3;
        }
        assert_eq!(Some(3), tree1.peek().map(|node| node.borrow().elem));
    }

    #[test]
    fn test_inorder_traverse() {
        let tree1 = Tree::from_vec(vec![2, 1, 3, 2]);
        assert_eq!(vec![1, 2, 2, 3], tree1.inorder_traverse());

        let mut tree2 = Tree::from_vec(vec![2, 1, 3, 2]);
        tree2.insert(4);
        assert_eq!(vec![1, 2, 2, 3, 4], tree2.inorder_traverse());
    }

    //    1
    //      2
    //        3
    //      2   4
    #[test]
    fn test_level_traverse() {
        let tree1 = Tree::from_vec(vec![1, 2, 3, 4, 2]);
        assert_eq!(vec![1, 2, 3, 2, 4], tree1.level_traverse());

        let tree2 = Tree::from_vec(vec![1, 2, 3, 2, 4]);
        assert_eq!(vec![1, 2, 3, 2, 4], tree2.level_traverse());

        let mut tree3 = Tree::from_vec(vec![1, 2, 3, 2, 4]);
        tree3.insert(0);
        assert_eq!(vec![1, 0, 2, 3, 2, 4], tree3.level_traverse());
    }

    //    1
    //      2
    //        3
    //      2   4
    #[test]
    fn test_dfs_traverse() {
        let tree1 = Tree::from_vec(vec![1, 2, 3, 4, 2]);
        assert_eq!(
            vec![vec![1, 2, 3, 2], vec![1, 2, 3, 4]],
            tree1.dfs_traverse()
        );
    }

    #[test]
    fn test_find() {
        let mut tree1 = Tree::from_vec(vec![2, 1, 3, 2]);
        assert!(tree1.find(2));
        assert!(tree1.find(3));
        assert_eq!(false, tree1.find(4));

        tree1.insert(4);
        assert!(tree1.find(4));
    }

    #[test]
    fn test_largest() {
        let tree1 = Tree::from_vec(vec![2, 1, 3, 2, 4]);
        assert_eq!(Some(4), tree1.largest());
        assert_eq!(Some(4), tree1.k_largest(1));
        assert_eq!(Some(3), tree1.k_largest(2));
    }
}
