// https://codeforces.com/problemset/problem/50/A

use crate::{Scanner, Solution};
use std::cmp::{max, min};
use std::io::{Read, Write};

#[derive(Default)]
pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        let m = arr[0];
        let n = arr[1];

        let res = get_cover_num(m, n);
        write!(output, "{}", res).ok();
    }
}

// the maxium ways to cover a mxn rectangle using 1x2 bricks
fn get_cover_num(m: usize, n: usize) -> usize {
    let smaller = min(m, n);
    let bigger = max(m, n);
    if smaller == 0 {
        return 0;
    } else if smaller == 1 {
        return bigger / 2;
    } else if smaller == 2 {
        return bigger;
    }
    let q = smaller / 2;
    let r = smaller % 2;
    q * bigger + r * (bigger / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![["2 4", "4"], ["3 3", "4"], ["1 5", "2"], ["16 16", "128"]];
        test_helper(cases, Solution1::default());
    }
}
