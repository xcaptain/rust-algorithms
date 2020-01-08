// https://leetcode-cn.com/problems/arithmetic-slices/

pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; a.len()];
    let mut res = 0;
    for i in 2..a.len() {
        if a[i] - a[i - 1] == a[i - 1] - a[i - 2] {
            dp[i] = 1 + dp[i - 1];
            res += dp[i];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p413() {
        assert_eq!(3, number_of_arithmetic_slices(vec![1, 2, 3, 4]));
        assert_eq!(10, number_of_arithmetic_slices(vec![1, 2, 3, 4, 5, 6]));
    }
}
// [1,2,3] 4;123,1234,12345,123456
// [2,3,4] 2;234,2345,23456
// [3,4,5] 1;345,3456
// [1,3,5] 1;135
// [2,4,6] 1;246
