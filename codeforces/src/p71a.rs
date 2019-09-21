// https://codeforces.com/contest/71/problem/A

use crate::Scanner;
use std::io::{Read, Write};

fn abbr(s: String) -> String {
    let l = s.len();
    if l > 10 {
        return format!(
            "{}{}{}",
            s.chars().nth(0).unwrap(),
            l - 2,
            s.chars().nth(l - 1).unwrap()
        );
    }
    s
}

pub fn solution_of_p71a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scan = Scanner::new(input);

    let n = scan.next_line();
    for _i in 0..n {
        let line = scan.next_line::<String>();
        let r = abbr(line);
        writeln!(out, "{}", r).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_abbr() {
        assert_eq!(
            String::from("i18n"),
            abbr(String::from("internationalization"))
        );
    }

    #[test]
    fn test_solution_of_p71a() {
        let cases = vec![[
            "4
word
localization
internationalization
pneumonoultramicroscopicsilicovolcanoconiosis
",
            "word
l10n
i18n
p43s",
        ]];

        test_helper(cases, solution_of_p71a);
    }
}
