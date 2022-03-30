pub fn is_palindrome(n: usize) -> bool {
    if n < 10 {
        return true;
    }
    let rem = n % 10;
    let mut q = n;

    let mut exp: u32 = 0;
    while q >= 10 {
        q /= 10;
        exp += 1;
    }
    if q != rem {
        return false;
    }

    let new_n = (n - q * (10_u32.pow(exp) as usize) - rem) / 10;

    // println!("new_n: {}", new_n);

    is_palindrome(new_n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(3));
        assert!(is_palindrome(121));
        assert!(is_palindrome(888));
        assert_eq!(false, is_palindrome(678));
        assert!(is_palindrome(67876));
    }
}
