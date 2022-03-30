// https://leetcode-cn.com/problems/integer-to-roman/

pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut res = String::new();

    while num > 0 {
        if num >= 1000 {
            res.push('M');
            num -= 1000;
        } else if num >= 900 {
            res.push_str("CM");
            num -= 900;
        } else if num >= 500 {
            res.push('D');
            num -= 500;
        } else if num >= 400 {
            res.push_str("CD");
            num -= 400;
        } else if num >= 100 {
            res.push('C');
            num -= 100;
        } else if num >= 90 {
            res.push_str("XC");
            num -= 90;
        } else if num >= 50 {
            res.push('L');
            num -= 50;
        } else if num >= 40 {
            res.push_str("XL");
            num -= 40;
        } else if num >= 10 {
            res.push('X');
            num -= 10;
        } else if num >= 9 {
            res.push_str("IX");
            num -= 9;
        } else if num >= 5 {
            res.push('V');
            num -= 5;
        } else if num >= 4 {
            res.push_str("IV");
            num -= 4;
        } else {
            res.push('I');
            num -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p12() {
        assert_eq!("III", int_to_roman(3));
        assert_eq!("IV", int_to_roman(4));
        assert_eq!("IX", int_to_roman(9));
        assert_eq!("LVIII", int_to_roman(58));
        assert_eq!("MCMXCIV", int_to_roman(1994));
        assert_eq!("MCMXCV", int_to_roman(1995));
        assert_eq!("MCMXCVI", int_to_roman(1996));
        assert_eq!("MCMXCIX", int_to_roman(1999));
    }
}
