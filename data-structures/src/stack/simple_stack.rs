/// a simple stack
/// it's much easier than queue, because just need one pointer
/// sp means stack pointer which points to the top node of a stack

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Stack<T> {
    sp: Link<T>,
    length: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            sp: None,
            length: 0,
        }
    }

    pub fn push(&mut self, t: T) {
        let node = Box::new(Node {
            elem: t,
            next: self.sp.take(),
        });
        self.sp = Some(node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.sp.take() {
            None => None,
            Some(node) => {
                let node = *node;
                self.sp = node.next;
                self.length -= 1;
                Some(node.elem)
            }
        }
        // use map rather than match
        // self.sp.take().map(|node| {
        //     let node = *node;
        //     self.sp = node.next;
        //     node.elem
        // })
    }

    /// return the reference of the top element of the stack
    pub fn top(&self) -> Option<&T> {
        self.sp.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.sp.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_basic() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        let n = s.pop();
        assert_eq!(Some(2), n);
        assert_eq!(1, s.length);
    }

    #[test]
    fn test_top() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        let t = s.top();
        assert_eq!(Some(&2), t);
    }

    #[test]
    fn test_top_mut() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);

        // TODO: figure out how to modify the stack top element
        let t = s.top_mut();
        assert_eq!(Some(&mut 2), t);
    }
}