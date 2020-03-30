//! This problem was asked by Facebook.
//! Given the mapping a = 1, b = 2, ... z = 26, and an encoded message, count the number of ways it can be decoded.
//! For example, the message '111' would give 3, since it could be decoded as 'aaa', 'ka', and 'ak'.
//! You can assume that the messages are decodable. For example, '001' is not allowed.

pub fn number_decode(s: String) -> usize {
    let l = s.len();
    let mut dp = vec![0; l + 1]; // dp[i] is decode number of s[0..i]
    dp[0] = 1;
    dp[1] = 1;

    for i in 1..l {
        if &s[i..=i] == "0" {
            if &s[i - 1..=i - 1] == "1" || &s[i - 1..=i - 1] == "2" {
                dp[i + 1] = dp[i - 1]; // s[i] can't use singly, must use s[i-1..i], 10, 20
            } else {
                return 0; // 30, 40... illeagle, return 0
            }
        } else {
            if &s[i - 1..=i - 1] == "1" || (&s[i - 1..=i - 1] == "2" && &s[i..=i] <= "6") {
                dp[i + 1] = dp[i] + dp[i - 1]; // 1x, 2..6, 2 methods adds up
            } else {
                dp[i + 1] = dp[i]; // 0x, 27, 28, 29, 3x..
            }
        }
    }

    dp[l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_decode() {
        assert_eq!(3, number_decode(String::from("111")));
    }
}
