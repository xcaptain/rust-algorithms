// a Queue implementation copied from stdlib
// I don't know why it's so hard to implement a queue in rust
use std::cell::UnsafeCell;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    next: AtomicPtr<Node<T>>,
    value: Option<T>,
}

pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: UnsafeCell<*mut Node<T>>,
}

impl<T> Node<T> {
    unsafe fn new(v: Option<T>) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            next: AtomicPtr::new(ptr::null_mut()),
            value: v,
        }))
    }
}

pub enum PopResult<T> {
    /// Some data has been popped
    Data(T),
    /// The queue is empty
    Empty,
    /// The queue is in an inconsistent state. Popping data should succeed, but
    /// some pushers have yet to make enough progress in order allow a pop to
    /// succeed. It is recommended that a pop() occur "in the near future" in
    /// order to see if the sender has made progress or not
    Inconsistent,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let stub = unsafe { Node::new(None) };
        Queue {
            head: AtomicPtr::new(stub),
            tail: UnsafeCell::new(stub),
        }
    }

    pub fn push(&self, t: T) {
        unsafe {
            let n = Node::new(Some(t));
            let prev = self.head.swap(n, Ordering::AcqRel);
            (*prev).next.store(n, Ordering::Release);
        }
    }

    pub fn pop(&self) -> PopResult<T> {
        unsafe {
            let tail = *self.tail.get();
            let next = (*tail).next.load(Ordering::Acquire);

            if !next.is_null() {
                *self.tail.get() = next;
                assert!((*tail).value.is_none());
                assert!((*next).value.is_some());
                let ret = (*next).value.take().unwrap();
                let _: Box<Node<T>> = Box::from_raw(tail);
                return PopResult::Data(ret);
            }

            if self.head.load(Ordering::Acquire) == tail {
                PopResult::Empty
            } else {
                PopResult::Inconsistent
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            let mut cur = *self.tail.get();
            while !cur.is_null() {
                let next = (*cur).next.load(Ordering::Relaxed);
                let _: Box<Node<T>> = Box::from_raw(cur);
                cur = next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PopResult, Queue};
    #[test]
    fn test_full() {
        let q: Queue<Box<i32>> = Queue::new();
        q.push(Box::new(1));
        q.push(Box::new(2));
        let n = q.pop();
        match n {
            PopResult::Data(t) => {
                assert_eq!(Box::new(1), t);
            }
            _ => panic!("pop result should be integer"),
        }
    }
}
