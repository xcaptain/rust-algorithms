//! rust blocking queue, a better implementation can be found at
//! https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-queue/src/array_queue.rs
pub struct BlockingQueue<T> {
    pub data: Vec<T>,
    pub is_blocking: bool,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        BlockingQueue {
            data: Vec::new(),
            is_blocking: false,
        }
    }

    /// push element into the queue, if the queue is blocked, then return error
    /// if not we can do the push and set status to blocked
    pub fn push(&mut self, t: T) -> Result<(), &str> {
        if self.is_blocking {
            return Err("queue is blocked, can't push");
        }
        self.data.insert(0, t);
        self.is_blocking = true;

        Ok(())
    }

    /// pop from a queue, if it's not blocked then return error
    pub fn pop(&mut self) -> Result<T, &str> {
        if !self.is_blocking {
            return Err("queue is not blocked, can't pop");
        }
        self.is_blocking = false;
        Ok(self.data.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread::spawn;

    #[test]
    fn test_blocking_queue() {
        let q = Arc::new(Mutex::new(BlockingQueue::new()));
        // create a thread, main thread push and another thread pop
        q.as_ref().lock().unwrap().push(1).unwrap();
        spawn(move || {
            assert_eq!(
                1,
                q.as_ref().lock().map(|mut n| { n.pop().unwrap() }).unwrap()
            );
        })
        .join()
        .unwrap();
    }
}
