// https://leetcode-cn.com/problems/third-maximum-number/

use std::cmp::Ordering;

pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(a)); // 降序
    nums.dedup();
    if nums.len() < 3 {
        return nums[0];
    }
    nums[2]
}

pub fn third_max_v2(nums: Vec<i32>) -> i32 {
    let mut first = i32::min_value();
    let mut second = i32::min_value();
    let mut third = i32::min_value();

    for num in nums {
        if num > third {
            match num.cmp(&second) {
                Ordering::Less => {
                    third = num;
                }
                Ordering::Greater => match num.cmp(&first) {
                    Ordering::Less => {
                        third = second;
                        second = num;
                    }
                    Ordering::Greater => {
                        third = second;
                        second = first;
                        first = num;
                    }
                    Ordering::Equal => {}
                },
                Ordering::Equal => {}
            }
        }
    }
    if third == i32::min_value() {
        return first;
    }
    third
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p414() {
        assert_eq!(1, third_max(vec![3, 2, 1]));
        assert_eq!(1, third_max(vec![2, 2, 3, 1]));
        assert_eq!(2, third_max(vec![1, 2]));

        assert_eq!(1, third_max_v2(vec![3, 2, 1]));
        assert_eq!(1, third_max_v2(vec![2, 2, 3, 1]));
        assert_eq!(2, third_max_v2(vec![1, 2]));
    }
}
