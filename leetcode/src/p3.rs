// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/

use std::cmp::max;
use std::collections::HashSet;

/// brute force
pub fn length_of_longest_substring_v1(s: String) -> i32 {
    let l = s.len();
    if l <= 1 {
        return l as i32;
    }
    let mut ans = 1;
    for i in 0..l - 1 {
        for j in i + 1..l {
            if s[i..j].contains(&s[j..=j]) {
                break;
            }
            ans = max(ans, j - i + 1);
        }
    }
    ans as i32
}

/// 滑动窗口法
pub fn length_of_longest_substring_v2(s: String) -> i32 {
    let mut set: HashSet<char> = HashSet::new();
    let l = s.len();
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    while i < l && j < l {
        let c = s.chars().nth(j).unwrap();
        if !set.contains(&c) {
            set.insert(c);
            j += 1;
            ans = max(ans, j - i);
        } else {
            let left_c = s.chars().nth(i).unwrap();
            set.remove(&left_c);
            i += 1;
        }
    }

    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p3() {
        assert_eq!(3, length_of_longest_substring_v1(String::from("abcabcbb")));
        assert_eq!(1, length_of_longest_substring_v1(String::from("bbbbb")));
        assert_eq!(3, length_of_longest_substring_v1(String::from("pwwkew")));

        assert_eq!(3, length_of_longest_substring_v2(String::from("abcabcbb")));
        assert_eq!(1, length_of_longest_substring_v2(String::from("bbbbb")));
        assert_eq!(3, length_of_longest_substring_v2(String::from("pwwkew")));
    }
}
