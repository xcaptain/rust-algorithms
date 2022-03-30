// https://leetcode-cn.com/problems/increasing-triplet-subsequence/

/// straight forward, stupid but works
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let l = nums.len();
    if l < 3 {
        return false;
    }
    for i in 0..l - 2 {
        for j in i + 1..l - 1 {
            if nums[i] >= nums[j] {
                continue;
            }
            for k in j + 1..l {
                if nums[j] < nums[k] {
                    return true;
                }
            }
        }
    }
    false
}

pub fn increasing_triplet_one_loop(nums: Vec<i32>) -> bool {
    let mut small = i32::max_value();
    let mut mid = i32::max_value();
    for num in nums {
        if num <= small {
            small = num;
        } else if num <= mid {
            mid = num;
        } else if num > mid {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p334() {
        assert_eq!(false, increasing_triplet(vec![1]));
        assert!(increasing_triplet(vec![1, 2, 3, 4, 5]));
        assert_eq!(false, increasing_triplet(vec![5, 4, 3, 2, 1]));
        assert_eq!(false, increasing_triplet(vec![1, 1, -2, 6]));

        assert_eq!(false, increasing_triplet_one_loop(vec![1]));
        assert!(increasing_triplet_one_loop(vec![1, 2, 3, 4, 5]));
        assert_eq!(false, increasing_triplet_one_loop(vec![5, 4, 3, 2, 1]));
        assert_eq!(false, increasing_triplet_one_loop(vec![1, 1, -2, 6]));
    }
}
