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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        assert_eq!(true, is_palindrome(0));
        assert_eq!(true, is_palindrome(121));
        assert_eq!(false, is_palindrome(-121));
    }
}
