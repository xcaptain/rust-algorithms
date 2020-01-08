// https://leetcode-cn.com/problems/top-k-frequent-elements/

use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq)]
struct FreqKV(i32, i32);

impl Ord for FreqKV {
    fn cmp(&self, other: &Self) -> Ordering {
        // desc order
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for FreqKV {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl PartialEq for FreqKV {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 && self.0 == other.0
    }
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let counter = map.entry(num).or_insert(0);
        *counter += 1;
    }

    // by default we create a max-heap
    let mut heap = BinaryHeap::new();
    for (key, num) in map {
        heap.push(FreqKV(key, num));
        if heap.len() > k as usize {
            heap.pop();
        }
        // println!("heap peek: {}", heap.peek().unwrap().0);
    }
    let mut res = vec![];
    while !heap.is_empty() {
        let value = heap.pop().unwrap();
        res.push(value.0);
    }

    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p347() {
        assert_eq!(vec![1, 2], top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
        assert_eq!(vec![1], top_k_frequent(vec![1], 1));
    }
}
