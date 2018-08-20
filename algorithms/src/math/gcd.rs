pub fn gcd(m: usize, n: usize) -> usize {
    let g = m % n;
    if g == 0 {
        return n;
    }
    return gcd(n, g);
}

pub fn gcd_2(mut m: usize, mut n: usize) -> usize {
    while m != n {
        if m == 0 {
            return n;
        }
        if n == 0 {
            return m;
        }

        if m > n {
            m = m - n;
        } else {
            n = n - m;
        }
    }
    return m;
}

pub fn gcd_3(mut m: usize, mut n: usize) -> usize {
    while n != 0 {
        let t = n;
        n = m % n;
        m = t;
    }
    return m;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(1, gcd(1, 2));
        assert_eq!(1, gcd_2(1, 2));
        assert_eq!(1, gcd_3(1, 2));
    }

    #[test]
    fn test_normal_reverse() {
        assert_eq!(1, gcd(2, 1));
        assert_eq!(1, gcd_2(2, 1));
        assert_eq!(1, gcd_3(2, 1));
    }

    #[test]
    fn test_mut_prime() {
        assert_eq!(1, gcd(3, 2));
        assert_eq!(1, gcd_2(3, 2));
        assert_eq!(1, gcd_3(3, 2));
    }

    #[test]
    fn test_mut_prime_reverse() {
        assert_eq!(1, gcd(2, 3));
        assert_eq!(1, gcd_2(2, 3));
        assert_eq!(1, gcd_3(2, 3));
    }

    #[test]
    fn test_divide() {
        assert_eq!(2, gcd(2, 4));
        assert_eq!(2, gcd_2(2, 4));
        assert_eq!(2, gcd_3(2, 4));
    }

    #[test]
    fn test_divide_reverse() {
        assert_eq!(2, gcd(4, 2));
        assert_eq!(2, gcd_2(4, 2));
        assert_eq!(2, gcd_3(4, 2));
    }

    #[test]
    fn test_zero() {
        assert_eq!(2, gcd(0, 2));
        assert_eq!(2, gcd_2(0, 2));
        assert_eq!(2, gcd_3(0, 2));
    }

    #[test]
    fn test_large_distance() {
        // performance test
        assert_eq!(1, gcd(1, 90000000000));
        // too slow because of too many divisions
        // assert_eq!(1, gcd_2(1, 90000000000));
        assert_eq!(1, gcd_3(1, 90000000000));
    }
}
