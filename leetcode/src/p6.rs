// https://leetcode-cn.com/problems/zigzag-conversion/solution/zzi-xing-bian-huan-by-jyd/

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
        return s;
    }
    let mut res = vec![String::new(); num_rows as usize];
    let mut i: i32 = 0;
    let mut flag: i32 = -1;
    for c in s.chars() {
        res[i as usize] += &c.to_string();
        if i == num_rows - 1 || i == 0 {
            flag = -flag;
        }
        i += flag;
    }

    res.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p6() {
        assert_eq!(
            String::from("LCIRETOESIIGEDHN"),
            convert(String::from("LEETCODEISHIRING"), 3)
        );
        assert_eq!(String::from("AB"), convert(String::from("AB"), 1));
    }
}
