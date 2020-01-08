// https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mut m = HashMap::new();
    m.insert("2", vec!["a", "b", "c"]);
    m.insert("3", vec!["d", "e", "f"]);
    m.insert("4", vec!["g", "h", "i"]);
    m.insert("5", vec!["j", "k", "l"]);
    m.insert("6", vec!["m", "n", "o"]);
    m.insert("7", vec!["p", "q", "r", "s"]);
    m.insert("8", vec!["t", "u", "v"]);
    m.insert("9", vec!["w", "x", "y", "z"]);

    let mut ans = VecDeque::new();
    ans.push_back(String::new());
    for j in 0..digits.len() {
        let s = &digits[j..=j];
        let arr = m.get(&s).unwrap();
        for _i in 0..ans.len() {
            let p = ans.pop_front().unwrap();
            for c in arr {
                ans.push_back(p.clone() + c);
            }
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p17() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            letter_combinations(String::from("23"))
        );
        assert_eq!(
            vec![String::new(); 0],
            letter_combinations(String::from(""))
        );
    }
}
