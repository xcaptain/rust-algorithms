use std::cell::RefCell;
use std::cmp::Ord;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode<T: Ord + Debug + Clone> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Rc<RefCell<TreeNode<T>>>>;

pub struct Tree<T: Ord + Debug + Clone> {
    root: Link<T>,
    total_nodes: i32,
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
        if self.elem == t {
            return true;
        } else if self.elem < t {
            return match self.right.as_ref() {
                None => false,
                Some(node) => node.borrow().find(t),
            };
        } else {
            return match self.left.as_ref() {
                None => false,
                Some(node) => node.borrow().find(t),
            };
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
    pub fn inorder_traverse(&self) -> Vec<T> {
        // TODO: use &Option<Rc<RefCell<TreeNode<T>>>> or Option<&Rc<RefCell<TreeNode<T>>>>?
        // what's the difference?
        let mut res = vec![];
        Self::inorder_traverse_helper(self.root.as_ref(), &mut res);
        res
    }

    fn inorder_traverse_helper(root: Option<&Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        if let Some(cur_node) = root {
            let tree_node = cur_node.borrow_mut();
            if !tree_node.left.is_none() {
                Self::inorder_traverse_helper(tree_node.left.as_ref(), res);
            }
            // println!("elem: {:?}", tree_node.elem);
            res.push(tree_node.elem.clone());
            if !tree_node.right.is_none() {
                Self::inorder_traverse_helper(tree_node.right.as_ref(), res);
            }
        }
    }

    /// pick the root node of a tree, return a reference to the pointer
    pub fn peek(&self) -> Option<&Rc<RefCell<TreeNode<T>>>> {
        // self.root.as_ref().map(|node| node.borrow().elem.clone())
        self.root.as_ref()
    }

    pub fn find(&self, t: T) -> bool {
        if let Some(cur_node) = self.root.as_ref() {
            return cur_node.borrow().find(t);
        }
        false
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
        let root = tree1.peek();
        let root_elem = root.map(|node| node.borrow().elem);
        assert_eq!(Some(2), root_elem);
    }

    #[test]
    fn test_inorder_traverse() {
        let tree1 = Tree::from_vec(vec![2, 1, 3, 2]);
        assert_eq!(vec![1, 2, 2, 3], tree1.inorder_traverse());

        let mut tree2 = Tree::from_vec(vec![2, 1, 3, 2]);
        tree2.insert(4);
        assert_eq!(vec![1, 2, 2, 3, 4], tree2.inorder_traverse());
    }

    #[test]
    fn test_find() {
        let mut tree1 = Tree::from_vec(vec![2, 1, 3, 2]);
        assert_eq!(true, tree1.find(2));
        assert_eq!(true, tree1.find(3));
        assert_eq!(false, tree1.find(4));

        tree1.insert(4);
        assert_eq!(true, tree1.find(4));
    }
}
