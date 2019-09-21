pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut pre = String::from("");
    let mut i = 0;
    if strs.is_empty() {
        return pre;
    }
    loop {
        let first_s: Vec<char> = strs[0].chars().collect();
        if first_s.len() <= i {
            return pre;
        }
        let ic = first_s[i];
        for s in &strs {
            let vs: Vec<char> = s.chars().collect();
            if vs.len() <= i || ic != vs[i] {
                return pre;
            }
        }
        pre = format!("{}{}", pre, ic);
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
