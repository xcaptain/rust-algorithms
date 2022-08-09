// https://codeforces.com/problemset/problem/160/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let mut _n = scanner.next_line::<String>().parse::<usize>().unwrap();
        let mut arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        arr.sort_by(|a, b| b.cmp(a)); // asc order
        let res = minium_take(arr);
        write!(output, "{}", res).ok();
    }
}

fn minium_take(arr: Vec<usize>) -> usize {
    let total: usize = arr.iter().sum();
    let mut take_num = 0;
    let l = arr.len();
    for (i, item) in arr.iter().enumerate() {
        let rest = total - take_num;
        if take_num > rest {
            return i;
        }
        take_num += item;
    }
    l
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![
            [
                "2
3 3", "2",
            ],
            [
                "3
2 1 2", "2",
            ],
        ];
        test_helper(cases, Solution1);
    }
}
