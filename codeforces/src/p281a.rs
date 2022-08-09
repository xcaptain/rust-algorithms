// https://codeforces.com/problemset/problem/281/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let line1 = scanner.next_line::<String>();
        let res = capitalize_first_character(line1);
        write!(output, "{}", res).ok();
    }
}

fn capitalize_first_character(s: String) -> String {
    if s.is_empty() {
        return String::from("");
    }
    let ss = s.as_str();
    let head = &ss[0..1];
    let head_upper = head.to_uppercase();
    let rest_part = &ss[1..];
    format!("{}{}", head_upper, rest_part)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![["ApPLe", "ApPLe"], ["konjac", "Konjac"]];
        test_helper(cases, Solution1);
    }
}
