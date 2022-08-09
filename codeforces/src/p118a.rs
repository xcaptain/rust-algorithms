// https://codeforces.com/problemset/problem/118/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

#[derive(Default)]
pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let s1 = scanner.next_line::<String>();
        let mut arr1: Vec<char> = vec![];
        for c in s1.chars() {
            if c == 'a'
                || c == 'e'
                || c == 'i'
                || c == 'o'
                || c == 'u'
                || c == 'y'
                || c == 'A'
                || c == 'E'
                || c == 'I'
                || c == 'O'
                || c == 'U'
                || c == 'Y'
            {
                continue;
            }
            if c.is_uppercase() {
                arr1.push('.');
                arr1.push(c.to_ascii_lowercase());
            } else {
                arr1.push('.');
                arr1.push(c);
            }
        }
        let res = arr1.iter().collect::<String>();
        writeln!(output, "{}", res).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![
            ["tour", ".t.r"],
            ["Codeforces", ".c.d.f.r.c.s"],
            ["aBAcAba", ".b.c.b"],
        ];
        test_helper(cases, Solution1::default());
    }
}
