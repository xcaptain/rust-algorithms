// https://codeforces.com/problemset/problem/546/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        let k = arr[0];
        let n = arr[1];
        let w = arr[2];
        let mut s = 0;
        for i in 1..=w {
            s += i * k;
        }
        if s > n {
            write!(output, "{}", s - n).ok();
        } else {
            write!(output, "0").ok();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![["3 17 4", "13"]];
        test_helper(cases, Solution1);
    }
}
