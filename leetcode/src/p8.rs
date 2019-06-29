// https://leetcode-cn.com/problems/string-to-integer-atoi/

pub fn my_atoi(s: String) -> i32 {
    // check sign, positivs or negtive or invalid
    let mut sign = 1;
    let s = s.trim();
    let mut i = 0;
    for c in s.chars() {
        if c == '-' {
            sign = -1;
            i += 1;
            break;
        } else if c == '+' {
            i += 1;
            break;
        } else if c >= '0' && c <= '9' {
            // start with number, valid
            break;
        } else {
            return 0; // start with characters
        }
    }
    let mut result: i64 = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if c < '0' || c > '9' {
            break;
        } else {
            let d = ((c as u8) - ('0' as u8)) as i32 * sign;
            result = 10 * result + d as i64;
            if result <= (i32::min_value() as i64) {
                return i32::min_value();
            }
            if result >= (i32::max_value() as i64) {
                return i32::max_value();
            };
            i += 1;
        }
    }
    return result as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(10, my_atoi(String::from("10")));
        assert_eq!(10, my_atoi(String::from(" 10 ")));
        assert_eq!(10, my_atoi(String::from("10 a")));
        assert_eq!(0, my_atoi(String::from("- 10")));
        // would be much more complicated if - 10 --> -10
        // need to recognize wether space comes in start or middle
        // assert_eq!(0, my_atoi(String::from("- 10")));
        assert_eq!(-10, my_atoi(String::from("-10hello")));
        assert_eq!(4193, my_atoi(String::from("4193 with words")));
        assert_eq!(0, my_atoi(String::from("hello-10")));
        assert_eq!(0, my_atoi(String::from("-hello10")));
        assert_eq!(-2147483648, my_atoi(String::from("-91283472332")));
        assert_eq!(2147483647, my_atoi(String::from("111283472332")));
        assert_eq!(0, my_atoi(String::from("   +0 123")));
        assert_eq!(2147483647, my_atoi(String::from("2147483648")));
    }
}
