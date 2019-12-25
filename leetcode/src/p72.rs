// https://leetcode-cn.com/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode/

use std::cmp::min;

// dp, how to deal with boundary
// dp[i][0] = i means word1 with length i word2 is empty, so distance is i
pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let row: Vec<i32> = vec![0; n + 1];
    let mut dp: Vec<Vec<i32>> = vec![row; m + 1];
    for i in 0..=m {
        dp[i][0] = i as i32;
    }
    for j in 0..=n {
        dp[0][j] = j as i32;
    }

    for i in 1..=m {
        for j in 1..=n {
            if &word1[i - 1..i] == &word2[j - 1..j] {
                dp[i][j] = min(min(dp[i - 1][j] + 1, dp[i][j - 1] + 1), dp[i - 1][j - 1]);
            } else {
                dp[i][j] = min(
                    min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                    dp[i - 1][j - 1] + 1,
                );
            }
        }
    }
    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p72() {
        assert_eq!(3, min_distance(String::from("horse"), String::from("ros")));
    }
}
