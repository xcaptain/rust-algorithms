// https://leetcode-cn.com/problems/3sum/

use std::cmp::Ordering;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();
    let mut res = vec![];
    for i in 0..nums.len() - 2 {
        let mut l = i + 1;
        let mut r = nums.len() - 1;

        if nums[i] > 0 {
            return res;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        while l < r {
            let s: i32 = nums[i] + nums[l] + nums[r];
            match s.cmp(&0_i32) {
                Ordering::Equal => {
                    res.push(vec![nums[i], nums[l], nums[r]]);
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
    res
}

// the brute force, very slow, will time out
pub fn three_sum_v2(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let l = nums.len();
    let mut res = vec![];
    for i in 0..l - 2 {
        for j in i + 1..l - 1 {
            for k in j + 1..l {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut t = vec![nums[i], nums[j], nums[k]];
                    t.sort();
                    if !res.contains(&t) {
                        res.push(t);
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
    fn test_p15() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            three_sum(vec![-1, 0, 1, 2, -1, -4])
        );

        assert_eq!(vec![vec![0, 0, 0]], three_sum(vec![0, 0, 0]));

        assert_eq!(
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2]
            ],
            three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6])
        );
    }

    #[test]
    fn test_p15_v2() {
        assert_eq!(
            vec![vec![-1, 0, 1], vec![-1, -1, 2]],
            three_sum_v2(vec![-1, 0, 1, 2, -1, -4])
        );

        assert_eq!(vec![vec![0, 0, 0]], three_sum_v2(vec![0, 0, 0]));

        assert_eq!(
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2]
            ],
            three_sum_v2(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6])
        );
    }
}
