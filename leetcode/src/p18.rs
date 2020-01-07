// https://leetcode-cn.com/problems/4sum/

use std::cmp::Ordering;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();
    let mut res = vec![];
    for i in 0..nums.len() - 3 {
        if nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target {
            return res;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..nums.len() - 2 {
            let mut l = j + 1;
            let mut r = nums.len() - 1;

            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            while l < r {
                let s = nums[j] + nums[l] + nums[r];
                let exp_target = target - nums[i];
                match s.cmp(&exp_target) {
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    }
                    Ordering::Less => {
                        l += 1;
                    }
                    Ordering::Greater => {
                        r -= 1;
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p18() {
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        );
        assert_eq!(vec![vec![0, 0, 0, 0]], four_sum(vec![0, 0, 0, 0], 0));
        assert_eq!(
            vec![vec![-4, 0, 1, 2], vec![-1, -1, 0, 1]],
            four_sum(vec![-1, 0, 1, 2, -1, -4], -1)
        );
        assert_eq!(
            vec![
                vec![-3, -2, 2, 3],
                vec![-3, -1, 1, 3],
                vec![-3, 0, 0, 3],
                vec![-3, 0, 1, 2],
                vec![-2, -1, 0, 3],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1]
            ],
            four_sum(vec![-3, -2, -1, 0, 0, 1, 2, 3], 0)
        );
        assert_eq!(
            vec![vec![-5, -4, -3, 1]],
            four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11)
        );
        assert_eq!(
            vec![vec![-5, -5, -1, 4], vec![-5, -3, -1, 2]],
            four_sum(vec![-1, -5, -5, -3, 2, 5, 0, 4], -7)
        );
        assert_eq!(
            vec![
                vec![-5, -4, -1, 1],
                vec![-5, -4, 0, 0],
                vec![-5, -2, -2, 0],
                vec![-4, -2, -2, -1]
            ],
            four_sum(vec![-1, 0, -5, -2, -2, -4, 0, 1, -2], -9)
        );
    }
}
