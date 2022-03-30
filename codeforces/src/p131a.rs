// https://codeforces.com/problemset/problem/131/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p131a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let line = scanner.next_line::<String>();
    if line.len() == 1 {
        let c = line.chars().next().unwrap();
        if c.is_ascii_uppercase() {
            write!(out, "{}", line.to_ascii_lowercase()).ok();
        } else {
            write!(out, "{}", line.to_ascii_uppercase()).ok();
        }
    } else if only_uppercase(&line) {
        write!(out, "{}", line.to_ascii_lowercase()).ok();
    } else if uppercase_except_first(&line) {
        write!(
            out,
            "{}{}",
            &line[0..1].to_ascii_uppercase(),
            &line[1..].to_ascii_lowercase()
        )
        .ok();
    } else {
        write!(out, "{}", line).ok();
    }
}

fn only_uppercase(s: &str) -> bool {
    for c in s.chars() {
        if c.is_ascii_lowercase() {
            return false;
        }
    }
    true
}

fn uppercase_except_first(s: &str) -> bool {
    let first_c = s.chars().next().unwrap();
    if first_c.is_ascii_uppercase() {
        return false;
    }
    for c in (&s[1..]).chars() {
        if c.is_ascii_lowercase() {
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
    fn test_solution_of_p131a() {
        let cases = vec![["cAPS", "Caps"], ["Lock", "Lock"]];
        test_helper(cases, solution_of_p131a);
    }
}
