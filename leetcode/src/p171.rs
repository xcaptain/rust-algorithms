// https://leetcode-cn.com/problems/excel-sheet-column-number/

use std::collections::HashMap;

pub fn title_to_number(s: String) -> i32 {
    let table = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut v_map: HashMap<char, i32> = HashMap::new();
    for (index, c) in table.chars().enumerate() {
        v_map.insert(c, (index as i32) + 1);
    }
    let mut res: i32 = 0;
    for (index, c) in s.as_str().chars().enumerate() {
        let val = v_map.get(&c).unwrap();
        let e: u32 = (s.len() - index - 1) as u32;
        res += *val * 26i32.pow(e);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(1, title_to_number(String::from("A")));
        assert_eq!(28, title_to_number(String::from("AB")));
        assert_eq!(701, title_to_number(String::from("ZY")));
    }
}
