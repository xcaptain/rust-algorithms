// https://leetcode-cn.com/problems/lru-cache/

use std::collections::VecDeque;

/// TODO: use a linked hashmap so we get O(1)
pub struct LRUCache {
    pub capacity: usize,
    pub data: VecDeque<(i32, i32)>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            data: VecDeque::with_capacity(capacity as usize),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(pos) = self.data.iter().position(|&x| x.0 == key) {
            let data = self.data[pos];
            self.data.remove(pos);
            self.data.push_front(data);
            return data.1;
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(pos) = self.data.iter().position(|&x| x.0 == key) {
            self.data.remove(pos);
            self.data.push_front((key, value));
        } else {
            // 不在数组中，插入
            let l = self.data.len();
            if l >= self.capacity {
                self.data.pop_back();
                // println!("pop {}, {:?}, {:?}", l, t, (key, value));
                self.data.push_front((key, value));
            } else {
                self.data.push_front((key, value));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p146() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(1, cache.get(1));

        cache.put(3, 3);
        assert_eq!(-1, cache.get(2));

        cache.put(4, 4);
        assert_eq!(-1, cache.get(1));
        assert_eq!(3, cache.get(3));
        assert_eq!(4, cache.get(4));

        // cache2
        let mut cache2 = LRUCache::new(2);
        cache2.put(2, 1);
        cache2.put(1, 1);
        cache2.put(2, 3);
        cache2.put(4, 1);

        assert_eq!(-1, cache2.get(1));
        assert_eq!(3, cache2.get(2));
    }
}
