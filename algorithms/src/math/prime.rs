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

/// generate the first n primes
pub fn get_prime_table(n: usize) -> Vec<usize> {
    let mut num = 2;
    let mut index = 0;
    let mut arr = vec![];
    loop {
        if index >= n {
            break;
        }
        if is_prime(num) {
            arr.push(num);
            index += 1;
        }
        num += 1;
    }
    arr
}

/// get prime table less or equal than n
pub fn get_prime_table_le(n: usize) -> Vec<usize> {
    let mut num = 2;
    let mut arr = vec![];
    loop {
        if num > n {
            break;
        }
        if is_prime(num) {
            arr.push(num);
        }
        num += 1;
    }
    arr
}

/// get the prime factor array of num
pub fn get_prime_factors(num: usize) -> Vec<usize> {
    let mut n = num;
    let mut result = vec![];
    while n % 2 == 0 {
        result.push(2);
        n /= 2;
    }
    for i in (3..=((n as f64).sqrt() as usize)).step_by(2) {
        while n % i == 0 {
            result.push(i);
            n /= i;
        }
    }
    if n > 2 {
        result.push(n);
    }
    return result;
}

/// get unique prime factors of num
pub fn get_uniq_prime_factors(num: usize) -> Vec<usize> {
    let mut factors = get_prime_factors(num);
    factors.sort();
    factors.dedup();
    return factors;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(false, is_prime(4));
        assert_eq!(true, is_prime(5));
    }

    #[test]
    fn test_get_prime_table() {
        assert_eq!(vec![2, 3, 5], get_prime_table(3));
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
            get_prime_table(10)
        );
    }

    #[test]
    fn test_get_prime_table_le() {
        assert_eq!(vec![2, 3, 5], get_prime_table_le(5));
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(vec![2, 3, 5], get_prime_factors(30));
        assert_eq!(vec![2, 2, 3, 5], get_prime_factors(60));
        assert_eq!(vec![2, 2, 2, 5, 5, 5, 23], get_prime_factors(23000));
    }

    #[test]
    fn test_get_uniq_prime_factors() {
        assert_eq!(vec![2, 3, 5], get_uniq_prime_factors(30));
        assert_eq!(vec![2, 3, 5], get_uniq_prime_factors(60));
        assert_eq!(vec![2, 5, 23], get_uniq_prime_factors(23000));
    }

    // #[test]
    // fn test_very_large_prime() { // 暂时不测试大素数
    //     assert_eq!(true, is_prime(170141183460469231731687303715884105727));
    // }
}
