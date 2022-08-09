// https://codeforces.com/problemset/problem/236/A

use crate::{Scanner, Solution};
use std::collections::HashSet;
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let uniq_arr: HashSet<char> = scanner.next_line::<String>().chars().collect();
        let res = uniq_arr.len();
        if res % 2 == 1 {
            write!(output, "IGNORE HIM!").ok();
        } else {
            write!(output, "CHAT WITH HER!").ok();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![
            ["wjmzbmr", "CHAT WITH HER!"],
            ["xiaodao", "IGNORE HIM!"],
            ["sevenkplus", "CHAT WITH HER!"],
        ];
        test_helper(cases, Solution1);
    }
}
