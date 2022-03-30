// https://leetcode-cn.com/problems/water-and-jug-problem/

pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
    if x == 0 || y == 0 {
        return z == 0 || z == x + y;
    } else if z > x + y {
        return false;
    }

    z % gcd(x, y) == 0
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
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
    fn test_p365() {
        assert!(can_measure_water(3, 5, 4));
        assert_eq!(false, can_measure_water(2, 6, 5));
    }
}
