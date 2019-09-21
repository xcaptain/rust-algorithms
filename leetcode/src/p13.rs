// https://leetcode-cn.com/problems/roman-to-integer/

use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let c_map: HashMap<char, i32> = vec![
        ('V', 5),
        ('I', 1),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect();
    let mut i = 0;
    let mut result = 0;
    while i < s.len() {
        let current_char = s.chars().nth(i).unwrap();
        let current_num = *c_map.get(&current_char).unwrap();
        if i + 1 < s.len() {
            let next_char = s.chars().nth(i + 1).unwrap();
            let next_num = *c_map.get(&next_char).unwrap();
            if current_char == 'I' {
                if next_char == 'V' || next_char == 'X' {
                    result += next_num - current_num;
                    i += 2;
                } else if next_char == 'I' {
                    result += next_num + next_num;
                    i += 2;
                } else {
                    result += current_num;
                    i += 1;
                }
            } else if current_char == 'X' {
                if next_char == 'L' || next_char == 'C' {
                    result += next_num - current_num;
                    i += 2;
                } else if next_char == 'X' {
                    result += next_num + next_num;
                    i += 2;
                } else {
                    result += current_num;
                    i += 1;
                }
            } else if current_char == 'C' {
                if next_char == 'D' || next_char == 'M' {
                    result += next_num - current_num;
                    i += 2;
                } else if next_char == 'C' {
                    result += next_num + next_num;
                    i += 2;
                } else {
                    result += current_num;
                    i += 1;
                }
            } else {
                result += current_num;
                i += 1;
            }
        } else {
            result += current_num;
            i += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(1, roman_to_int(String::from("I")));
        assert_eq!(3, roman_to_int(String::from("III")));
        assert_eq!(4, roman_to_int(String::from("IV")));
        assert_eq!(8, roman_to_int(String::from("VIII")));
        assert_eq!(9, roman_to_int(String::from("IX")));
        assert_eq!(10, roman_to_int(String::from("X")));
        assert_eq!(58, roman_to_int(String::from("LVIII")));
        assert_eq!(1994, roman_to_int(String::from("MCMXCIV")));
    }
}
