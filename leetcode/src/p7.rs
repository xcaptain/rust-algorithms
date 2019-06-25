// leetcode p7 reverse an integer
// https://leetcode-cn.com/problems/reverse-integer/

pub fn reverse(x: i32) -> i32 {
    let mut num = x;
    let mut r: i64 = 0;
    let base: i32 = 10;
    if x > -10 && x < 10 {
        return x;
    }

    while num != 0 {
        let rem = (num % base) as i64;
        r = (r * 10) + rem;
        num = num / base;
    }
    if r > (i32::max_value() as i64) || r < (i32::min_value() as i64) {
        return 0;
    }
    return r as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(1, reverse(10));
        assert_eq!(21, reverse(12));
        assert_eq!(-21, reverse(-12));
        assert_eq!(-1, reverse(-1));
        assert_eq!(8001, reverse(1008));
    }
}
