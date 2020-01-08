// https://leetcode-cn.com/contest/weekly-contest-175/problems/minimum-number-of-steps-to-make-two-strings-anagram/

use std::collections::HashMap;

pub fn min_steps(s: String, t: String) -> i32 {
    let mut t_map = HashMap::new();
    for t_char in t.chars() {
        let counter = t_map.entry(t_char).or_insert(0);
        *counter += 1;
    }
    for c in s.chars() {
        let counter = t_map.entry(c).or_default();
        if *counter > 0 {
            *counter -= 1;
        }
    }
    let mut res = 0;
    for (_key, value) in t_map {
        res += value;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5333() {
        assert_eq!(1, min_steps(String::from("bab"), String::from("aba")));
        assert_eq!(
            5,
            min_steps(String::from("leetcode"), String::from("practice"))
        );
        assert_eq!(
            0,
            min_steps(String::from("anagram"), String::from("mangaar"))
        );
        assert_eq!(0, min_steps(String::from("xxyyzz"), String::from("xxyyzz")));
        assert_eq!(4, min_steps(String::from("friend"), String::from("family")));
        assert_eq!(
            18,
            min_steps(
                String::from("gctcxyuluxjuxnsvmomavutrrfb"),
                String::from("qijrjrhqqjxjtprybrzpyfyqtzf")
            )
        );
    }
}
