//! This problem was asked by Netflix.
//! Implement a queue using a set of fixed-length arrays.
//! The queue should support enqueue, dequeue, and get_size operations.

pub struct Queue<T: Clone> {
    pub data: Vec<T>,
    pub end_ptr: usize, // is a queue is full we should auto dequeue the first one
}

impl<T: Clone + Default> Queue<T> {
    pub fn new(length: usize) -> Self {
        Queue {
            data: vec![T::default(); length],
            end_ptr: 0,
        }
    }

    /// push an element to the end of the queue, if the queue
    /// is full then shift the left element out
    pub fn enqueue(&mut self, t: T) {
        if self.end_ptr < self.data.len() {
            self.data[self.end_ptr] = t;
            self.end_ptr += 1;
        } else {
            for i in 0..self.data.len() - 1 {
                self.data[i] = self.data[i + 1].clone();
            }
            let end = self.data.len() - 1;
            self.data[end] = t;
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.end_ptr == 0 {
            None
        } else {
            let first = self.data[0].clone();
            for i in 0..self.end_ptr - 1 {
                self.data[i] = self.data[i + 1].clone();
            }
            self.end_ptr -= 1;
            Some(first)
        }
    }

    pub fn get_size(&self) -> usize {
        self.end_ptr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q: Queue<usize> = Queue::new(3);
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        assert_eq!(3, q.get_size());

        let first = q.dequeue();
        assert_eq!(Some(1), first);
        assert_eq!(2, q.get_size());

        let second = q.dequeue();
        assert_eq!(Some(2), second);
        assert_eq!(1, q.get_size());

        let third = q.dequeue();
        assert_eq!(Some(3), third);
        assert_eq!(0, q.get_size());

        let forth = q.dequeue();
        assert_eq!(None, forth);
        assert_eq!(0, q.get_size());
    }

    #[test]
    fn test_full_enqueue() {
        let mut q: Queue<usize> = Queue::new(3);
        q.enqueue(1); // 1
        q.enqueue(2); // 1, 2
        q.enqueue(3); // 1, 2, 3
        q.enqueue(4); // 2, 3, 4

        assert_eq!(3, q.get_size());

        let first = q.dequeue();
        assert_eq!(Some(2), first);
        assert_eq!(2, q.get_size());
    }
}
