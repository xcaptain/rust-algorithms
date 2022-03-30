// https://leetcode-cn.com/problems/valid-palindrome/

pub fn is_palindrome(s: String) -> bool {
    let s = s.to_ascii_lowercase();
    let arr: Vec<char> = s.chars().collect();
    if arr.is_empty() {
        return true;
    }
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        if !arr[i].is_ascii_alphanumeric() {
            i += 1;
            continue;
        }
        if !arr[j].is_ascii_alphanumeric() {
            j -= 1;
            continue;
        }
        if arr[i] != arr[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(
            true,
            is_palindrome(String::from("A man, a plan, a canal: Panama"))
        );
        assert_eq!(false, is_palindrome(String::from("race a car")));
        assert!(is_palindrome(String::from("")));
    }
}
