use std::collections::HashMap;

/// 检查字符串s中是否有重复的字符
pub fn is_unique(s: String) -> bool {
    let mut m: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let value = m.entry(c).or_insert(0);
        *value += 1;
        if *value >= 2 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert_eq!(true, is_unique(String::from("abc")));
        assert_eq!(false, is_unique(String::from("abcb")));
        assert_eq!(true, is_unique(String::from("")));
        assert_eq!(true, is_unique(String::from("a")));
    }
}
