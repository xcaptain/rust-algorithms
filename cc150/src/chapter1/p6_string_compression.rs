use std::char;

pub fn compress(s1: String) -> String {
    let mut res = String::new();
    let mut curr_char = '0'; // a special sign, because s1 only contains a-zA-Z
    let mut curr_num = 0;

    for c in s1.chars() {
        if curr_char == '0' {
            curr_char = c;
            curr_num = 1;
            continue;
        }
        if c == curr_char {
            curr_num += 1;
        } else {
            if curr_num > 0 {
                res.push(curr_char);
                res.push(char::from_digit(curr_num, 10).unwrap());
            }
            curr_char = c;
            curr_num = 1;
        }
    }
    if curr_num > 0 && curr_char != '0' {
        res.push(curr_char);
        res.push(char::from_digit(curr_num, 10).unwrap());
    }

    if res.len() >= s1.len() {
        return s1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        assert_eq!("a2b1c5a3", compress(String::from("aabcccccaaa")));
        assert_eq!("abcdef", compress(String::from("abcdef")));
    }
}
