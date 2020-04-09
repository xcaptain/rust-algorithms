pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut pre = String::new();
    let mut i = 0;
    if strs.is_empty() {
        return pre;
    }
    loop {
        if strs[0].len() <= i {
            return pre;
        }
        let ic = &strs[0][i..=i]; // choose one string as base
        for s in &strs {
            // compare others with the base string
            if s.len() <= i || ic != &s[i..=i] {
                return pre;
            }
        }
        pre.push_str(ic);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            "fl".to_string(),
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );

        assert_eq!(
            "".to_string(),
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );

        assert_eq!("".to_string(), longest_common_prefix(vec![]));

        assert_eq!(
            "hello".to_string(),
            longest_common_prefix(vec!["hello".to_string()])
        );

        assert_eq!(
            "a".to_string(),
            longest_common_prefix(vec!["aa".to_string(), "a".to_string()])
        );
    }
}
