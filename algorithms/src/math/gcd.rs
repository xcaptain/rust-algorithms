/// use gcd1 as the default gcd algorithm
pub use gcd1 as gcd;

/// gcd algorithms using mod
pub fn gcd1(m: usize, n: usize) -> usize {
    let g = m % n;
    if g == 0 {
        return n;
    }
    gcd1(n, g)
}

/// euclid algorithms
pub fn gcd2(m: usize, n: usize) -> usize {
    let mut m = m;
    let mut n = n;
    while m != n {
        if m == 0 {
            return n;
        }
        if n == 0 {
            return m;
        }

        if m > n {
            m -= n;
        } else {
            n -= m;
        }
    }
    m
}

/// mod algorithm using iteration rather than recursion
pub fn gcd3(m: usize, n: usize) -> usize {
    let mut m = m;
    let mut n = n;
    while n != 0 {
        let t = n;
        n = m % n;
        m = t;
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(1, gcd1(1, 2));
        assert_eq!(1, gcd2(1, 2));
        assert_eq!(1, gcd3(1, 2));
    }

    #[test]
    fn test_normal_reverse() {
        assert_eq!(1, gcd1(2, 1));
        assert_eq!(1, gcd2(2, 1));
        assert_eq!(1, gcd3(2, 1));
    }

    #[test]
    fn test_mut_prime() {
        assert_eq!(1, gcd1(3, 2));
        assert_eq!(1, gcd2(3, 2));
        assert_eq!(1, gcd3(3, 2));
    }

    #[test]
    fn test_mut_prime_reverse() {
        assert_eq!(1, gcd1(2, 3));
        assert_eq!(1, gcd2(2, 3));
        assert_eq!(1, gcd3(2, 3));
    }

    #[test]
    fn test_divide() {
        assert_eq!(2, gcd1(2, 4));
        assert_eq!(2, gcd2(2, 4));
        assert_eq!(2, gcd3(2, 4));
    }

    #[test]
    fn test_divide_reverse() {
        assert_eq!(2, gcd1(4, 2));
        assert_eq!(2, gcd2(4, 2));
        assert_eq!(2, gcd3(4, 2));
    }

    #[test]
    fn test_zero() {
        assert_eq!(2, gcd1(0, 2));
        assert_eq!(2, gcd2(0, 2));
        assert_eq!(2, gcd3(0, 2));
    }

    #[test]
    fn test_large_distance() {
        // performance test
        assert_eq!(1, gcd(1, 90000000000));
        // too slow because of too many divisions
        // assert_eq!(1, gcd2(1, 90000000000));
        assert_eq!(1, gcd3(1, 90000000000));
    }
}
