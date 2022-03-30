// https://leetcode-cn.com/problems/decode-ways/

pub fn num_decodings(s: String) -> i32 {
    let l = s.len();
    if &s[0..=0] == "0" {
        return 0;
    }
    let mut dp = vec![0_i32; l + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 1..l {
        if &s[i..=i] == "0" {
            if &s[i - 1..=i - 1] == "1" || &s[i - 1..=i - 1] == "2" {
                dp[i + 1] = dp[i - 1];
            } else {
                return 0;
            }
        } else if &s[i - 1..=i - 1] == "1" || (&s[i - 1..=i - 1] == "2" && &s[i..=i] <= "6") {
            dp[i + 1] = dp[i] + dp[i - 1];
        } else {
            dp[i + 1] = dp[i];
        }
    }
    dp[l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p91() {
        assert_eq!(2, num_decodings(String::from("12")));
        assert_eq!(3, num_decodings(String::from("226")));
        assert_eq!(0, num_decodings(String::from("0")));
        assert_eq!(0, num_decodings(String::from("00")));
        assert_eq!(0, num_decodings(String::from("01")));
        assert_eq!(0, num_decodings(String::from("100")));
        assert_eq!(1, num_decodings(String::from("10")));
        assert_eq!(1, num_decodings(String::from("27")));
        assert_eq!(0, num_decodings(String::from("012")));
        assert_eq!(1, num_decodings(String::from("120")));
        assert_eq!(0, num_decodings(String::from("0120")));
    }
}
