// https://leetcode-cn.com/problems/find-the-duplicate-number/

use std::collections::HashMap;

// slow and stupid
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let counter = m.entry(num).or_insert(0);
        if counter == &1 {
            return num;
        }
        *counter += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p287() {
        assert_eq!(2, find_duplicate(vec![1, 3, 4, 2, 2]));
        assert_eq!(3, find_duplicate(vec![3, 1, 3, 4, 2]));
    }
}
