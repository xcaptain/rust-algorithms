// https://leetcode-cn.com/problems/longest-common-subsequence/

use std::cmp::max;

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let l1 = text1.len();
    let l2 = text2.len();
    let arr1: Vec<char> = text1.chars().collect();
    let arr2: Vec<char> = text2.chars().collect();
    let row = vec![0; l2 + 1];
    let mut dp: Vec<Vec<i32>> = vec![row; l1 + 1];
    for i in 1..=l1 {
        for j in 1..=l2 {
            if arr1[i - 1] == arr2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[l1][l2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1143() {
        assert_eq!(
            3,
            longest_common_subsequence(String::from("abcde"), String::from("ace"))
        );
        assert_eq!(
            3,
            longest_common_subsequence(String::from("abc"), String::from("abc"))
        );
        assert_eq!(
            0,
            longest_common_subsequence(String::from("abc"), String::from("def"))
        );
        assert_eq!(
            2,
            longest_common_subsequence(String::from("ezupkr"), String::from("ubmrapg"))
        );
        assert_eq!(
            4,
            longest_common_subsequence(
                String::from("pmjghexybyrgzczy"),
                String::from("hafcdqbgncrcbihkd")
            )
        );
    }
}
