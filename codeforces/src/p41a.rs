// http://codeforces.com/problemset/problem/41/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p41a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let s1 = scanner.next_line::<String>();
    let s2 = scanner.next_line::<String>();

    if is_translation(s1, s2) {
        write!(out, "YES").ok();
    } else {
        write!(out, "NO").ok();
    }
}

fn is_translation(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let l = s1.len();
    for i in 0..l {
        if s1[i..=i] != s2[l - i - 1..=l - i - 1] {
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
    fn test_solution_of_p41a() {
        let cases = vec![[
            "code
edoc",
            "YES",
        ]];
        test_helper(cases, solution_of_p41a);
    }
}
