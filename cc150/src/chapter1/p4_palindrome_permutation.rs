use std::collections::HashMap;

/// 校验这个字符串是否是一个回文串的排列
pub fn is_palindrome_permutation(s: String) -> bool {
    let s = s.to_ascii_lowercase();
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut l = 0;
    for c in s.chars() {
        if c == ' ' {
            continue;
        }
        let value = map.entry(c).or_insert(0);
        *value += 1;
        l += 1;
    }
    if l % 2 == 0 {
        // 字符串长度为偶数
        for (_k, v) in map {
            if v % 2 == 1 {
                // 如果有不匹配的，直接返回false
                return false;
            }
        }
    } else {
        let mut odd_num = 0;
        for (_k, v) in map {
            if v % 2 == 1 {
                odd_num += 1;
            }
        }
        if odd_num != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_permutation() {
        assert_eq!(true, is_palindrome_permutation(String::from("T a c t Coa")));
        assert_eq!(
            true,
            is_palindrome_permutation(String::from("jhsabckuj ahjsbckj"))
        );
        assert_eq!(
            true,
            is_palindrome_permutation(String::from("Able was I ere I saw Elba"))
        );
        assert_eq!(
            false,
            is_palindrome_permutation(String::from("So patient a nurse to nurse a patient so"))
        );
        assert_eq!(
            false,
            is_palindrome_permutation(String::from("Random Words"))
        );
    }
}
