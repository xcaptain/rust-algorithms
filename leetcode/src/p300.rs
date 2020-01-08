// https://leetcode-cn.com/problems/longest-increasing-subsequence/

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let l = nums.len();
    if l == 0 {
        return 0;
    }
    let mut dp = vec![0; l];
    dp[0] = 1;
    let mut res = 1;
    for i in 1..l {
        let mut maxval = 0;
        for j in 0..i {
            if nums[j] < nums[i] {
                maxval = maxval.max(dp[j]);
            }
        }
        dp[i] = maxval + 1;
        res = res.max(dp[i]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p300() {
        assert_eq!(3, length_of_lis(vec![10, 9, 2, 5, 3, 4]));
        assert_eq!(3, length_of_lis(vec![4, 10, 4, 3, 8, 9]));
        assert_eq!(1, length_of_lis(vec![10, 9, 2]));
        assert_eq!(4, length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
        assert_eq!(0, length_of_lis(vec![]));
        assert_eq!(1, length_of_lis(vec![2]));
    }
}
