// https://leetcode-cn.com/problems/plus-one/

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut digits_mut = digits;
    digits_mut.reverse();
    let mut adder = 1;
    for digit in digits_mut {
        if digit + adder >= 10 {
            res.insert(0, digit + adder - 10);
        } else {
            res.insert(0, digit + adder);
            adder = 0;
        }
    }
    // at last if adder is still 1, means the array will grow 1 digits
    if adder == 1 {
        res.insert(0, adder);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(vec![1, 2, 4], plus_one(vec![1, 2, 3]));
        assert_eq!(vec![4, 3, 2, 2], plus_one(vec![4, 3, 2, 1]));
        assert_eq!(vec![1, 0], plus_one(vec![9]));
    }
}
