// https://leetcode-cn.com/problems/palindrome-number/
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut n = x;
    let mut result = 0;
    while n > 0 {
        let last_digit = n % 10;
        result = result * 10 + last_digit;
        n /= 10;
    }
    result == x
}

pub fn is_palindrome_v2(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut reversed_num = 0;
    let mut n = x;
    while n > reversed_num {
        reversed_num = reversed_num * 10 + n % 10;
        n /= 10;
    }
    n == reversed_num || n == reversed_num / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p9() {
        assert!(is_palindrome(0));
        assert!(is_palindrome(121));
        assert_eq!(false, is_palindrome(-121));

        assert!(is_palindrome_v2(0));
        assert!(is_palindrome_v2(121));
        assert_eq!(false, is_palindrome_v2(-121));
    }
}
