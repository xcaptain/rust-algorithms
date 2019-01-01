// http://cglab.ca/~abeinges/blah/too-many-lists/book/
// https://doc.rust-lang.org/book/second-edition/ch15-01-box.html
// https://rustbyexample.com/custom_types/enum/testcase_linked_list.html

#[derive(Default)]
pub struct List<T> {
    length: usize,
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn create() -> Self {
        List {
            length: 0,
            head: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                let node = *node;
                self.head = node.next;
                self.length -= 1;
                Some(node.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // 从box中解出node本身，把指针清空，避免递归的清空资源
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Iterator, List};
    #[test]
    fn test_new() {
        let l: List<i32> = List::create();
        assert_eq!(true, l.head.is_none());
        assert_eq!(0, l.length);
    }

    #[test]
    fn test_push() {
        let mut l = List::create();
        l.push(10);
        assert_eq!(true, l.head.is_some());
        assert_eq!(1, l.length);
    }

    #[test]
    fn test_pop() {
        let mut l = List::create();
        l.push("ele1");
        let ele1 = l.pop();
        assert_eq!(0, l.length);
        assert_eq!(true, l.head.is_none());
        assert_eq!(Some("ele1"), ele1);
    }

    #[test]
    fn test_peak() {
        let mut l = List::create();
        l.push(1);
        l.push(2);
        l.push(3);
        let p = l.peek();
        assert_eq!(Some(&3), p);
    }

    #[test]
    fn test_peak_mut() {
        let mut l = List::create();
        l.push(1);
        l.push(2);
        l.push(3);
        let p = l.peek_mut();
        assert_eq!(Some(&mut 3), p);
    }

    #[test]
    fn test_into_iter() {
        let mut l = List::create();
        l.push(1);
        l.push(2);
        l.push(3);

        let mut iter = l.into_iter();
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());
    }
}
