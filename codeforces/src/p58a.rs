// https://codeforces.com/problemset/problem/58/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

#[derive(Default)]
pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let line1 = scanner.next_line::<String>();
        if contains_hello(line1) {
            write!(output, "YES").ok();
        } else {
            write!(output, "NO").ok();
        }
    }
}

fn contains_hello(s: String) -> bool {
    let word = "hello";
    let mut ss = &s[0..];
    for c in word.chars() {
        if let Some(cur_pos) = ss.find(c) {
            ss = &ss[cur_pos + 1..];
        } else {
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
        let cases = vec![["ahhellllloou", "YES"], ["hlelo", "NO"], ["hello", "YES"]];
        test_helper(cases, Solution1::default());
    }
}
