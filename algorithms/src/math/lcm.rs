use crate::math::gcd::gcd;

pub fn lcm(m: usize, n: usize) -> usize {
    let g = gcd(m, n);
    m * n / g
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
