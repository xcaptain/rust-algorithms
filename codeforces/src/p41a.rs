// http://codeforces.com/problemset/problem/41/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

#[derive(Default)]
pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let s1 = scanner.next_line::<String>();
        let s2 = scanner.next_line::<String>();

        if is_translation(s1, s2) {
            write!(output, "YES").ok();
        } else {
            write!(output, "NO").ok();
        }
    }
}

fn is_translation(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let l = s1.len();
    for i in 0..l {
        if s1[i..=i] != s2[l - i - 1..=l - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![[
            "code
edoc",
            "YES",
        ]];
        test_helper(cases, Solution1::default());
    }
}
