// https://leetcode-cn.com/problems/length-of-last-word/

// may try without `split_ascii_whitespace`
pub fn length_of_last_word(s: String) -> i32 {
    let mut res = 0;
    for word in s.split_ascii_whitespace() {
        res = word.len();
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(5, length_of_last_word(String::from("Hello World")));
    }
}
