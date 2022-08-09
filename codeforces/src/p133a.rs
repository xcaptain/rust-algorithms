// https://codeforces.com/problemset/problem/133/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let line = scanner.next_line::<String>();
        if line.contains('H') || line.contains('Q') || line.contains('9') {
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
        let cases = vec![["Hi!", "YES"], ["Codeforces", "NO"]];
        test_helper(cases, Solution1);
    }
}
