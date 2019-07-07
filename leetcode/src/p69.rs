// https://leetcode-cn.com/problems/sqrtx/

// algorithms from https://en.wikipedia.org/wiki/Methods_of_computing_square_roots
// but should be better algorithms that doesn't require guess to be i64
pub fn my_sqrt(x: i32) -> i32 {
    let mut guess: i64 = 1;
    let xx = x as i64;
    loop {
        if guess * guess <= xx && (guess + 1) * (guess + 1) > xx {
            break;
        }
        guess = (guess + xx / guess) / 2;
    }
    return guess as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(2, my_sqrt(4));
        assert_eq!(2, my_sqrt(8));
        assert_eq!(1, my_sqrt(3));
        assert_eq!(3, my_sqrt(10));
        assert_eq!(46340, my_sqrt(2147395600));
        assert_eq!(0, my_sqrt(0));
    }
}
