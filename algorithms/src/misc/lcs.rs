/// LCS means longest common subsequence
/// LCS Problem Statement: Given two sequences, find the length of longest subsequence present in both of them.
/// A subsequence is a sequence that appears in the same relative order, but not necessarily contiguous.
/// For example, “abc”, “abg”, “bdf”, “aeg”, ‘”acefg”, .. etc are subsequences of “abcdefg”.
/// So a string of length n has 2^n different possible subsequences.
/// https://www.geeksforgeeks.org/longest-common-subsequence-dp-4/
use std::cmp::max;

/// brute force method to get the lcs between s1 and s2
pub fn lcs1(s1: String, s2: String) -> usize {
    let m = s1.len();
    let n = s2.len();
    let mut l = vec![vec![0; n + 1]; m + 1]; // n+1 cols, m+1 rows

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                l[i][j] = 0;
            } else if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
                l[i][j] = l[i - 1][j - 1] + 1
            } else {
                l[i][j] = max(l[i - 1][j], l[i][j - 1]);
            }
        }
    }
    l[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs1() {
        assert_eq!(3, lcs1(String::from("ABCDGH"), String::from("AEDFHR")));
        assert_eq!(4, lcs1(String::from("AGGTAB"), String::from("GXTXAYB")));
    }
}
