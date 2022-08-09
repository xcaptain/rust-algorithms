// https://codeforces.com/problemset/problem/131/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let line = scanner.next_line::<String>();
        if line.len() == 1 {
            let c = line.chars().next().unwrap();
            if c.is_ascii_uppercase() {
                write!(output, "{}", line.to_ascii_lowercase()).ok();
            } else {
                write!(output, "{}", line.to_ascii_uppercase()).ok();
            }
        } else if only_uppercase(&line) {
            write!(output, "{}", line.to_ascii_lowercase()).ok();
        } else if uppercase_except_first(&line) {
            write!(
                output,
                "{}{}",
                &line[0..1].to_ascii_uppercase(),
                &line[1..].to_ascii_lowercase()
            )
            .ok();
        } else {
            write!(output, "{}", line).ok();
        }
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
    fn solution1() {
        let cases = vec![["cAPS", "Caps"], ["Lock", "Lock"]];
        test_helper(cases, Solution1);
    }
}
