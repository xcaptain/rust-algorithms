pub fn reverse_vowels(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let arr: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = cs.len() - 1;
    while i < j {
        if arr.contains(&cs[i]) {
            if arr.contains(&cs[j]) {
                cs.swap(i, j);
                i += 1;
                j -= 1;
            } else {
                j -= 1;
            }
        } else if arr.contains(&cs[j]) {
            i += 1;
        } else {
            i += 1;
            j -= 1;
        }
    }
    cs.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p345() {
        assert_eq!("holle", reverse_vowels(String::from("hello")));
        assert_eq!("leotcede", reverse_vowels(String::from("leetcode")));
        assert_eq!("i", reverse_vowels(String::from("i")));
        assert_eq!("", reverse_vowels(String::from("")));
    }
}
