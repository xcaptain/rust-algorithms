// https://leetcode-cn.com/problems/add-digits/

pub fn add_digits(num: i32) -> i32 {
    let mut res = num;
    while res >= 10 {
        let mut p = res;
        let mut s = 0;
        while p > 0 {
            let q = p % 10;
            s += q;
            p /= 10;
        }
        if s < 10 {
            return s;
        }
        res = s;
    }

    res
}

pub fn add_digits_recursive(num: i32) -> i32 {
    if num < 10 {
        return num;
    }
    let mut num = num;
    let mut s = 0;
    while num > 0 {
        let q = num % 10;
        s += q;
        num /= 10;
    }
    add_digits_recursive(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p258() {
        assert_eq!(2, add_digits_recursive(38));
        assert_eq!(2, add_digits(38));

        assert_eq!(1, add_digits_recursive(10));
        assert_eq!(1, add_digits(10));
    }
}
