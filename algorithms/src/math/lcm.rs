use crate::math::gcd::{gcd_3, gcd_4};

pub fn lcm(m: usize, n: usize) -> usize {
    let g = gcd_3(m, n);
    return m * n / g;
}

pub fn lcm1(m: u128, n: u128) -> u128 {
    let g = gcd_4(m, n);
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
