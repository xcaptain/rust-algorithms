// https://codeforces.com/problemset/problem/122/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let num = scanner.next_line::<String>().parse::<usize>().unwrap();
        // because the input is between 1 and 1000, so we can list all the lucky numbers
        let lucky_nums = vec![4, 7, 44, 47, 74, 77, 444, 447, 474, 477, 744, 747, 774, 777];
        if is_almose_lucky(num, lucky_nums) {
            write!(output, "YES").ok();
        } else {
            write!(output, "NO").ok();
        }
    }
}

fn is_almose_lucky(num: usize, arr: Vec<usize>) -> bool {
    if arr.contains(&num) {
        return true;
    }
    for item in &arr {
        if num % (*item) == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution() {
        let cases = vec![["47", "YES"], ["16", "YES"], ["78", "NO"]];
        test_helper(cases, Solution1);
    }
}
