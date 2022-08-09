// https://codeforces.com/problemset/problem/266/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let _n = scanner.next_line::<String>().parse::<usize>().unwrap();
        let arr: Vec<char> = scanner.next_line::<String>().chars().collect();
        let res = min_move_ways(arr);
        write!(output, "{}", res).ok();
    }
}

fn min_move_ways(mut arr: Vec<char>) -> usize {
    if arr.len() <= 1 {
        return 0;
    } else if arr.len() == 2 {
        if arr[0] == arr[1] {
            return 1;
        }
        return 0;
    }
    let mut i = 0;
    let mut res = 0;
    while i < arr.len() - 2 {
        if arr[i] == arr[i + 1] || arr[i + 1] == arr[i + 2] {
            arr.remove(i + 1);
            res += 1;
        } else {
            i += 1;
        }
    }
    if arr.len() <= 1 {
        return res;
    } else if arr.len() == 2 {
        if arr[0] == arr[1] {
            return res + 1;
        }
        return res;
    }
    res
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
RRG", "1",
            ],
            [
                "5
RRRRR
", "4",
            ],
            [
                "4
BRBG
", "0",
            ],
            [
                "5
BRBGG
", "1",
            ],
        ];
        test_helper(cases, Solution1);
    }
}
