// https://codeforces.com/problemset/problem/96/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

#[derive(Default)]
pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let s = scanner.next_line::<String>();
        if s.contains("0000000") || s.contains("1111111") {
            write!(output, "YES").ok();
        } else {
            write!(output, "NO").ok();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![["001001", "NO"], ["1000000001", "YES"]];

        test_helper(cases, Solution1::default());
    }
}
