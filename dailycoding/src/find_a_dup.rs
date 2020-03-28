//! This problem was asked by Google.
//! You are given an array of length n + 1 whose elements belong to the set {1, 2, ..., n}.
//! By the pigeonhole principle, there must be a duplicate. Find it in linear time and space.
//!
//! Hint:
//! use hashmap or bit array

use std::collections::HashMap;

/// use a hashmap, O(n) in time complexy and O(n) in space
pub fn find_a_dup_v1(arr: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in arr {
        let counter = map.entry(num).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            return num;
        }
    }
    0
}

/// use bytes array
pub fn find_a_dup_v2(arr: Vec<i32>) -> i32 {
    let n = arr.len() - 1;
    let bin_val = 2_u32.pow(n as u32) - 1;
    // 0000...11111
    let mut bin_arr = bin_val.to_le_bytes(); // in little endian display: [31, 0, 0, 0]
                                             // let mut bin_arr = bin_val.to_be_bytes(); // in big endian display: [0, 0, 0, 31]
                                             // println!("{:?}", bin_arr);

    for i in 1..=n {
        if bin_arr[i - 1] == 0 {
            return i as i32;
        } else {
            bin_arr[i - 1] = 1_u8;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_a_dup() {
        assert_eq!(2, find_a_dup_v1(vec![1, 2, 3, 4, 5, 2]));

        assert_eq!(2, find_a_dup_v2(vec![1, 2, 3, 4, 5, 2]));
    }
}
