// https://leetcode-cn.com/contest/weekly-contest-171/problems/convert-integer-to-the-sum-of-two-no-zero-integers/

pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    for i in 1..n {
        let other = n - i;
        if is_no_zero_integer(i) && is_no_zero_integer(other) {
            return vec![i, other];
        }
    }
    return vec![];
}

fn is_no_zero_integer(mut n: i32) -> bool {
    while n > 0 {
        if n % 10 == 0 {
            return false;
        }
        n /= 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5307() {
        assert_eq!(vec![1, 1], get_no_zero_integers(2));
        assert_eq!(vec![2, 9], get_no_zero_integers(11));
        assert_eq!(vec![1, 9999], get_no_zero_integers(10000));
        assert_eq!(vec![1, 68], get_no_zero_integers(69));
        assert_eq!(vec![11, 999], get_no_zero_integers(1010));
    }

    #[test]
    fn test_is_no_zero_integer() {
        assert_eq!(true, is_no_zero_integer(9));
        assert_eq!(false, is_no_zero_integer(1010));
    }
}
