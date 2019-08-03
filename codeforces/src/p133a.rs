// https://codeforces.com/problemset/problem/133/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p133a(input: &mut Read, out: &mut Write) {
    let mut scanner = Scanner::new(input);
    let line = scanner.next::<String>();
    if line.contains('H') || line.contains('Q') || line.contains('9') {
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
    fn test_solution_of_p133a() {
        let cases = vec![["Hi!", "YES"], ["Codeforces", "NO"]];
        test_helper(cases, solution_of_p133a);
    }
}
