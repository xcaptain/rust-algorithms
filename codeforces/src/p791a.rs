use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let a = arr[0];
        let b = arr[1];

        let n = get_n(a, b);
        write!(output, "{}", n).ok();
    }
}

fn get_n(a: usize, b: usize) -> u32 {
    // a*3^n > b*2^n
    for n in 1..100 {
        if a * 3_usize.pow(n) > b * 2_usize.pow(n) {
            return n;
        }
    }
    100
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![["4 7", "2"], ["4 9", "3"], ["1 1", "1"]];
        test_helper(cases, Solution1);
    }
}
