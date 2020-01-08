// https://leetcode-cn.com/problems/first-missing-positive/

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let l = nums.len();
    for i in 0..l {
        while nums[i] != (i + 1) as i32 {
            if nums[i] <= 0 || nums[i] > l as i32 || nums[i] == nums[(nums[i] - 1) as usize] {
                break;
            }
            // swap nums[i] and nums[nums[i]=1]
            let idx = (nums[i] - 1) as usize;
            nums.swap(i, idx);
        }
    }
    for i in 0..l {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    (l + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p41() {
        assert_eq!(3, first_missing_positive(vec![1, 2, 0]));
        assert_eq!(2, first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(1, first_missing_positive(vec![7, 8, 9, 11, 12]));
    }
}
