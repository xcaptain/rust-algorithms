// https://codeforces.com/problemset/problem/263/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let mut mat: Vec<Vec<usize>> = vec![];
        for _i in 0..5 {
            let arr: Vec<usize> = scanner
                .next_line::<String>()
                .split(' ')
                .map(|e| e.parse::<usize>().unwrap())
                .collect();
            mat.push(arr);
        }

        // find the cordinate of the 1 in the matrix
        let mut x = 0;
        let mut y = 0;
        for (i, mat_item) in mat.iter().enumerate().take(5) {
            for (j, elem) in mat_item.iter().enumerate().take(5) {
                if elem == &1 {
                    x = i;
                    y = j;
                }
            }
        }
        let mut res = 0;
        if x > 2 {
            res += x - 2;
        } else {
            res += 2 - x;
        }
        if y > 2 {
            res += y - 2;
        } else {
            res += 2 - y;
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
                "0 0 0 0 0
0 0 0 0 1
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
",
                "3",
            ],
            [
                "0 0 0 0 0
0 0 0 0 0
0 1 0 0 0
0 0 0 0 0
0 0 0 0 0
",
                "1",
            ],
        ];

        test_helper(cases, Solution1);
    }
}
