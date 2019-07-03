// https://leetcode-cn.com/problems/maximum-subarray/
use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let l = nums.len();
    let mut largest = i32::min_value();
    let mut sum = 0;

    for i in nums.iter().take(l) {
        sum = max(*i, sum + *i);
        largest = max(largest, sum);
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(6, max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    }
}
