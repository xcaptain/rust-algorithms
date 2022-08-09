// https://codeforces.com/problemset/problem/467/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let n = scanner.next_line::<String>().parse::<usize>().unwrap();
        let mut res = 0;
        for _i in 0..n {
            let arr: Vec<usize> = scanner
                .next_line::<String>()
                .split(' ')
                .map(|e| e.parse::<usize>().unwrap())
                .collect();
            let p = arr[0];
            let q = arr[1];
            if q - p >= 2 {
                res += 1;
            }
        }
        write!(output, "{}", res).ok();
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
                "3
1 1
2 2
3 3",
                "0",
            ],
            [
                "3
1 10
0 10
10 10",
                "2",
            ],
        ];
        test_helper(cases, Solution1);
    }
}
