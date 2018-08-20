// 最小公倍数算法

use math::gcd::gcd;

// 利用最大公约数计算
pub fn lcm(m: usize, n: usize) -> usize {
    let g = gcd(m, n);
    return m * n / g;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, lcm(1, 2));
    }

    #[test]
    fn test_zero() {
        assert_eq!(0, lcm(0, 10));
    }
}
