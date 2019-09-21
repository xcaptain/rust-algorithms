// https://codeforces.com/problemset/problem/112/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p112a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let arr1: Vec<char> = scanner
        .next_line::<String>()
        .to_ascii_lowercase()
        .chars()
        .collect();
    let arr2: Vec<char> = scanner
        .next_line::<String>()
        .to_ascii_lowercase()
        .chars()
        .collect();

    let res = lexicographical_order(arr1, arr2);
    writeln!(out, "{}", res).ok();
}

fn lexicographical_order(arr1: Vec<char>, arr2: Vec<char>) -> i32 {
    let l = arr1.len(); // assume the 2 string has the same length
    for i in 0..l {
        let u1 = arr1[i] as u8;
        let u2 = arr2[i] as u8;
        if u1 < u2 {
            return -1;
        } else if u1 > u2 {
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p112a() {
        let cases = vec![
            [
                "aaaa
aaaA
",
                "0",
            ],
            [
                "abs
Abz
", "-1",
            ],
            [
                "abcdefg
AbCdEfF
",
                "1",
            ],
        ];

        test_helper(cases, solution_of_p112a);
    }
}
