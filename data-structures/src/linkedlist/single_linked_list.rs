// http://cglab.ca/~abeinges/blah/too-many-lists/book/
// https://doc.rust-lang.org/book/second-edition/ch15-01-box.html
// https://rustbyexample.com/custom_types/enum/testcase_linked_list.html

pub struct SingleLinkedList {
    length: usize,
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl SingleLinkedList {
    pub fn new() -> Self {
        return SingleLinkedList {
            length: 0,
            head: None,
        };
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
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
}

impl Drop for SingleLinkedList {
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
    use super::*;
    #[test]
    fn test_new() {
        let l = SingleLinkedList::new();
        // assert_eq!(l.head, None);
        assert_eq!(0, l.length);
    }

    #[test]
    fn test_push() {
        let mut l = SingleLinkedList::new();
        l.push(10);
        assert_eq!(1, l.length);
    }

    #[test]
    fn test_pop() {
        let mut l = SingleLinkedList::new();
        l.push(10);
        let _ = l.pop();
        assert_eq!(0, l.length);
        // assert_eq!(None, l.head);
    }
}
