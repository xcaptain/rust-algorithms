// https://leetcode-cn.com/contest/weekly-contest-172/problems/maximum-69-number/

pub fn maximum69_number(num: i32) -> i32 {
    let s = num.to_string();
    let l = s.len();
    let mut arr: Vec<i32> = vec![num];
    for (i, c) in s.chars().enumerate() {
        let q: u32 = (l - i - 1) as u32;
        let v = 3 * 10_usize.pow(q) as i32;
        if c == '9' {
            arr.push(num - v);
        } else if c == '6' {
            arr.push(num + v);
        }
    }
    *arr.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5315() {
        assert_eq!(9969, maximum69_number(9669));
        assert_eq!(9999, maximum69_number(9996));
        assert_eq!(9999, maximum69_number(9999));
    }
}
