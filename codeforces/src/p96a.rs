// https://codeforces.com/problemset/problem/96/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p96a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let s = scanner.next_line::<String>();
    if s.contains("0000000") || s.contains("1111111") {
        write!(out, "YES").ok();
    } else {
        write!(out, "NO").ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p96a() {
        let cases = vec![["001001", "NO"], ["1000000001", "YES"]];

        test_helper(cases, solution_of_p96a);
    }
}
