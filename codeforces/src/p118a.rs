// https://codeforces.com/problemset/problem/118/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p118a(input: &mut dyn Read, out: &mut dyn Write) {
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
    writeln!(out, "{}", res).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p118a() {
        let cases = vec![
            ["tour", ".t.r"],
            ["Codeforces", ".c.d.f.r.c.s"],
            ["aBAcAba", ".b.c.b"],
        ];
        test_helper(cases, solution_of_p118a);
    }
}
