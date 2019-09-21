// https://leetcode-cn.com/problems/add-binary/

use std::cmp::max;

pub fn add_binary(a: String, b: String) -> String {
    let mut res = String::from("");
    let mut adder = 0;
    // the key to this problem is to pad a string, format_spec is odd
    // https://doc.rust-lang.org/std/fmt/index.html#syntax
    let m_len = max(a.len(), b.len());
    let aa = format!("{:0>width$}", a, width = m_len);
    let bb = format!("{:0>width$}", b, width = m_len);
    for i in (0..m_len).rev() {
        let av = (aa.chars().nth(i).unwrap() as u8) - b'0';
        let bv = (bb.chars().nth(i).unwrap() as u8) - b'0';
        let t = av + bv + adder;
        if t >= 2 {
            res.insert(0, (t - 2).to_string().chars().nth(0).unwrap());
            adder = 1;
        } else {
            res.insert(0, t.to_string().chars().nth(0).unwrap());
            adder = 0;
        }
    }
    if adder == 1 {
        res.insert(0, adder.to_string().chars().nth(0).unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(
            String::from("100"),
            add_binary(String::from("11"), String::from("1"))
        );
        assert_eq!(
            String::from("10101"),
            add_binary(String::from("1010"), String::from("1011"))
        );
    }
}
