// https://leetcode-cn.com/problems/longest-palindromic-substring/

use std::cmp::max;

pub fn longest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let mut start = 0_usize;
    let mut end = 0_usize;
    let l = s.len();
    for i in 0..l {
        // 查找中心
        let len1 = expand_around_center(&s, i, i);
        let len2 = expand_around_center(&s, i, i + 1);
        let len = max(len1, len2);
        if len > end - start {
            end = i + len / 2;
            start = i - (len - 1) / 2;
        }
    }
    String::from(&s[start..=end])
}

// use s.chars().nth(l) is slow here
fn expand_around_center(s: &str, left: usize, right: usize) -> usize {
    let mut l = left as i32;
    let mut r = right as i32;
    while l >= 0 && r < s.len() as i32 && s[l as usize..=l as usize] == s[r as usize..=r as usize] {
        l -= 1;
        r += 1;
    }
    (r - l - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5() {
        assert_eq!(
            String::from("aba"),
            longest_palindrome(String::from("babad"))
        );
    }
}
