// https://codeforces.com/problemset/problem/281/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p281a(input: &mut Read, out: &mut Write) {
    let mut scanner = Scanner::new(input);
    let line1 = scanner.next::<String>();
    let res = capitalize_first_character(line1);
    write!(out, "{}\n", res).ok();
}

fn capitalize_first_character(s: String) -> String {
    if s.is_empty() {
        return String::from("");
    }
    let ss = s.as_str();
    let head = &ss[0..1];
    let head_upper = head.to_uppercase();
    let rest_part = &ss[1..];
    return format!("{}{}", head_upper, rest_part);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p281a() {
        let cases = vec![["ApPLe", "ApPLe"], ["konjac", "Konjac"]];
        test_helper(cases, solution_of_p281a);
    }
}
