use std::cmp::max;

/// 问题来自leetcoder https://leetcode.com/problems/add-two-numbers/
/// 题目没什么大难度，主要是数据结构的构造
pub fn add_two_sum(l1: Vec<usize>, l2: Vec<usize>) -> Vec<usize> {
    let len1 = l1.len();
    let len2 = l2.len();
    let max_len = max(len1, len2);
    let mut carry = 0;

    let mut result = vec![];
    for i in 0..max_len {
        let v1 = match l1.get(i) {
            Some(&_x) => _x,
            None => 0,
        };
        let v2 = match l2.get(i) {
            Some(&_x) => _x,
            None => 0,
        };
        let mut r = v1 + v2 + carry;
        if r > 9 {
            r -= 10;
            carry = 1;
        } else {
            carry = 0;
        }
        result.push(r);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        assert_eq!(vec![7, 0, 8], add_two_sum(vec![2, 4, 3], vec![5, 6, 4]));
    }
}
