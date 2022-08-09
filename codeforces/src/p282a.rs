// https://codeforces.com/problemset/problem/282/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let n = scanner.next_line::<String>().parse::<usize>().unwrap();
        let mut val = 0;
        for _i in 0..n {
            let statement = scanner.next_line::<String>();
            if statement.contains("++") {
                val += 1;
            } else if statement.contains("--") {
                val -= 1;
            } else {
                panic!("unimplemented statement");
            }
        }
        write!(output, "{}", val).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![
            [
                "1
++X", "1",
            ],
            [
                "2
X++
--X",
                "0",
            ],
        ];

        test_helper(cases, Solution1);
    }
}
