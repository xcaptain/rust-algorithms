// https://leetcode-cn.com/problems/k-diff-pairs-in-an-array/

pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = 0;
    let mut i = 0;
    let l = nums.len();
    if l == 0 {
        return 0;
    }
    while i < l - 1 {
        if i >= 1 && nums[i] == nums[i - 1] {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < l && nums[j] - nums[i] < k {
            j += 1;
        }
        if j != l && nums[j] - nums[i] == k {
            res += 1;
        }
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p532() {
        assert_eq!(2, find_pairs(vec![3, 1, 4, 1, 5], 2));
        assert_eq!(4, find_pairs(vec![1, 2, 3, 4, 5], 1));
        assert_eq!(1, find_pairs(vec![1, 3, 1, 5, 4], 0));
        assert_eq!(1, find_pairs(vec![1, 1, 1, 2, 1], 1));
        assert_eq!(0, find_pairs(vec![], 1));
        assert_eq!(0, find_pairs(vec![1], 1));
    }
}
