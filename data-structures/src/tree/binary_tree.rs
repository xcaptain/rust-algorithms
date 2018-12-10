// binary search tree
// define tree node first, and then define link rules
use std::cmp::PartialOrd;
use std::fmt::Debug;

struct Node<T: PartialOrd + Debug> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Tree<T: PartialOrd + Debug> {
    root: Link<T>,
}

impl<T: PartialOrd + Debug> Node<T> {
    pub fn new(t: T) -> Self {
        Node {
            elem: t,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, t: T) {
        if self.elem > t {
            // insert left
            match self.left.take() {
                None => {
                    // left tree is empty, use this node as root node
                    self.left = Some(Box::new(Node::new(t)));
                }
                Some(mut node) => {
                    (*node).insert(t);
                    self.left = Some(node); // gives back ownership to self.left
                }
            }
        } else {
            match self.right.take() {
                None => {
                    self.right = Some(Box::new(Node::new(t)));
                }
                Some(mut node) => {
                    (*node).insert(t);
                    self.right = Some(node);
                }
            }
        }
    }
}

impl<T: PartialOrd + Debug + Copy> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, t: T) {
        // find a position to insert t
        match self.root.take() {
            None => {
                let root_node = Box::new(Node::new(t));
                self.root = Some(root_node);
                println!("insert into root {:?}", t);
            }
            Some(node) => {
                let mut node = *node;
                &node.insert(t);
                println!("cur node {:?}, insert elem {:?}", node.elem, t.clone());
                self.root = Some(Box::new(node));
            }
        }
    }

    // pick the root node of a tree
    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.elem)
    }

    // find if element t exists in tree
    pub fn find(&self, t: T) -> bool {
        let mut cur = self.root.as_ref();
        while let Some(cur_node) = cur {
            if cur_node.elem == t {
                return true;
            } else if cur_node.elem > t {
                let left_node = &cur_node.left;
                match left_node {
                    None => {
                        return false;
                    }
                    Some(left_node_ptr) => {
                        cur = Some(left_node_ptr);
                    }
                }
            } else {
                let right_node = &cur_node.right;
                match right_node {
                    None => {
                        return false;
                    }
                    Some(right_node_ptr) => {
                        cur = Some(right_node_ptr);
                    }
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t: Tree<i32> = Tree::new();
        assert_eq!(true, t.root.is_none());
    }

    #[test]
    fn test_insert() {
        let mut t = Tree::new();
        t.insert(1);
        t.insert(2);
        let root_elem = t.peek();
        assert_eq!(Some(&1), root_elem);
    }

    #[test]
    fn test_find() {
        let mut t = Tree::new();
        t.insert(1);
        t.insert(2);
        assert_eq!(true, t.find(1));
        assert_eq!(false, t.find(3));
    }
}
