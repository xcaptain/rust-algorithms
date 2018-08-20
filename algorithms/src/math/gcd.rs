// 最大公约数算法，m,n >= 0
// TODO: 如果m,n差距很大的话会有性能问题，需要找个更加优化的算法
pub fn gcd(m: usize, n: usize) -> usize {
    let g = m % n;
    if g == 0 {
        return n;
    }
    return gcd(n, g);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(1, gcd(1, 2));
    }

    #[test]
    fn test_normal_reverse() {
        assert_eq!(1, gcd(1, 2));
    }

    #[test]
    fn test_mut_prime() {
        assert_eq!(1, gcd(3, 2));
    }

    #[test]
    fn test_mut_prime_reverse() {
        assert_eq!(1, gcd(2, 3));
    }

    #[test]
    fn test_divide() {
        assert_eq!(2, gcd(2, 4));
    }

    #[test]
    fn test_divide_reverse() {
        assert_eq!(2, gcd(4, 2));
    }

    #[test]
    fn test_zero() {
        assert_eq!(2, gcd(0, 2));
    }

    #[test]
    fn test_large_distance() {
        // 性能测试
        assert_eq!(1, gcd(1, 90000000000));
    }
}
