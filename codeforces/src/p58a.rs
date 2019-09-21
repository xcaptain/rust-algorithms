// https://codeforces.com/problemset/problem/58/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p58a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let line1 = scanner.next_line::<String>();
    if contains_hello(line1) {
        write!(out, "YES").ok();
    } else {
        write!(out, "NO").ok();
    }
}

fn contains_hello(s: String) -> bool {
    let word = "hello";
    let mut ss = &s[0..];
    for c in word.chars() {
        if let Some(cur_pos) = ss.find(c) {
            ss = &ss[cur_pos + 1..];
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p58a() {
        let cases = vec![["ahhellllloou", "YES"], ["hlelo", "NO"], ["hello", "YES"]];
        test_helper(cases, solution_of_p58a);
    }
}
