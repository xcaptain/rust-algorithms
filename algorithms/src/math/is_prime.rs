pub fn is_prime(n: usize) -> bool {
    let m = (n as f64).sqrt();
    let end = (m as usize) + 1;
    for i in 2..end {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime_1() {
        assert_eq!(true, is_prime(2));
    }

    #[test]
    fn test_is_prime_2() {
        assert_eq!(true, is_prime(3));
    }

    #[test]
    fn test_is_prime_3() {
        assert_eq!(false, is_prime(4));
    }

    #[test]
    fn test_is_prime_4() {
        assert_eq!(true, is_prime(5));
    }
    // #[test]
    // fn test_very_large_prime() { // 暂时不测试大素数
    //     assert_eq!(true, is_prime(170141183460469231731687303715884105727));
    // }
}
