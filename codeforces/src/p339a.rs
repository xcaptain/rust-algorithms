// https://codeforces.com/problemset/problem/339/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let line1 = scanner.next_line::<String>();
        let mut arr: Vec<usize> = line1
            .split('+')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        arr.sort_unstable(); // asc

        let arr2: Vec<String> = arr.into_iter().map(|e: usize| e.to_string()).collect();
        let res = arr2.join("+");
        write!(output, "{}", res).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![["3+2+1", "1+2+3"], ["1+1+3+1+3", "1+1+1+3+3"], ["2", "2"]];
        test_helper(cases, Solution1);
    }
}
