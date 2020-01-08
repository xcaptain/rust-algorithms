// https://leetcode-cn.com/problems/3sum-closest/

use std::cmp::{Ord, Ordering};

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut ans = nums[0] + nums[1] + nums[2];
    let mut nums = nums;
    nums.sort(); // 升序
    let l = nums.len();
    for i in 0..l {
        let mut left = i + 1;
        let mut right = l - 1;
        while left < right {
            let s = nums[i] + nums[left] + nums[right];
            if (s - target).abs() < (ans - target).abs() {
                ans = s;
            }
            match s.cmp(&target) {
                Ordering::Less => {
                    left += 1;
                }
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Equal => {
                    return ans;
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p16() {
        assert_eq!(2, three_sum_closest(vec![-1, 2, 1, -4], 1));
        assert_eq!(2, three_sum_closest(vec![1, 1, 1, 0], -100));
        assert_eq!(0, three_sum_closest(vec![0, 2, 1, -3], 1));
    }
}
