use std::collections::HashMap;

pub fn is_permutation(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut m1: HashMap<char, i32> = HashMap::new();
    for c in s1.chars() {
        let value = m1.entry(c).or_insert(0);
        *value += 1;
    }

    let mut m2: HashMap<char, i32> = HashMap::new();
    for c in s2.chars() {
        let value = m2.entry(c).or_insert(0);
        *value += 1;
    }

    for (k1, v1) in m1 {
        if let Some(v2) = m2.get(&k1) {
            if v1 != *v2 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(
            true,
            is_permutation(String::from("abc"), String::from("cab"))
        );
        assert_eq!(
            false,
            is_permutation(String::from("abc"), String::from("ab"))
        );
        assert_eq!(
            false,
            is_permutation(String::from("abb"), String::from("ab"))
        );
    }
}
