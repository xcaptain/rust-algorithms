/// check if s2 is a rotation of s1 using only is_substring method
pub fn string_rotation(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() || s1.is_empty() {
        return false;
    }
    let mut s3 = s1.clone();
    s3.push_str(&s1);

    is_substring(s3, s2)
}

/// check if s2 is a substring of s1
fn is_substring(s1: String, s2: String) -> bool {
    match s1.find(&s2) {
        Some(_pos) => true,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_rotation() {
        // waterbottlewaterbottle  erbottlewat
        assert!(string_rotation(
            String::from("waterbottle"),
            String::from("erbottlewat")
        ));
    }
}
