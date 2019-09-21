// https://leetcode-cn.com/problems/factorial-trailing-zeroes/

// 1*2*3...n
// how many 5 in this sequence
// 5 * 8 : 1 five
// 5 * 80 = 5 * 8 * 2 * 5: 2 five
pub fn trailing_zeroes(n: i32) -> i32 {
    let mut n = n;
    let mut num = 0;
    while n >= 5 {
        num += n / 5;
        n /= 5;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(1, trailing_zeroes(5));
        assert_eq!(0, trailing_zeroes(3));
    }
}
