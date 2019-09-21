// https://codeforces.com/problemset/problem/546/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p546a(input: &mut dyn Read, out: &mut dyn Write) {
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
        write!(out, "{}", s - n).ok();
    } else {
        write!(out, "0").ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p546a() {
        let cases = vec![["3 17 4", "13"]];
        test_helper(cases, solution_of_p546a);
    }
}
