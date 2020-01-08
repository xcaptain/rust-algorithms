// https://leetcode-cn.com/problems/longest-palindrome/

use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }
    let mut res = 0;
    let mut one_count = 0;
    for (_c, count) in map {
        if count % 2 == 0 {
            res += count;
        } else if count > 1 && count % 2 == 1 {
            res += count - 1;
            one_count += 1;
        } else if count == 1 {
            one_count += 1;
        }
    }
    if one_count >= 1 {
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p409() {
        assert_eq!(7, longest_palindrome(String::from("abccccdd")));
        assert_eq!(3, longest_palindrome(String::from("ccc")));
    }
}
