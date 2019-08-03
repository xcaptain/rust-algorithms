// https://leetcode-cn.com/problems/house-robber/

use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut pre_max = 0;
    let mut cur_max = 0;
    for i in 0..nums.len() {
        let temp = cur_max;
        cur_max = max(pre_max + nums[i], cur_max);
        pre_max = temp;
    }
    return cur_max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(4, rob(vec![1, 2, 3, 1]));
        assert_eq!(12, rob(vec![2, 7, 9, 3, 1]));
    }
}
