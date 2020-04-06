// https://leetcode-cn.com/problems/single-number/

use std::collections::HashSet;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let uniq_nums: HashSet<i32> = nums.iter().cloned().collect();
    let mut s1 = 0;
    for x in uniq_nums.iter() {
        s1 += x;
    }
    let mut s2 = 0;
    for x in nums.iter() {
        s2 += x;
    }
    2 * s1 - s2
}

pub fn single_number_v2(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for num in nums {
        res ^= num;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(1, single_number(vec![2, 2, 1]));
        assert_eq!(4, single_number(vec![4, 1, 2, 1, 2]));

        assert_eq!(1, single_number_v2(vec![2, 2, 1]));
        assert_eq!(4, single_number_v2(vec![4, 1, 2, 1, 2]));
        assert_eq!(-1, single_number_v2(vec![1, 3, 1, -1, 3]));
    }
}
