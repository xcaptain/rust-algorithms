// https://leetcode-cn.com/problems/majority-element/

use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mid = (nums.len() / 2 + 1) as i32;
    let mut n_map: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        let counter = n_map.entry(n).or_insert(0);
        *counter += 1;
        if *counter >= mid {
            return n;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(3, majority_element(vec![3, 2, 3]));
        assert_eq!(2, majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
