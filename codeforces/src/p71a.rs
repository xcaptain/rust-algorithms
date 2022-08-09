// https://codeforces.com/contest/71/problem/A

use crate::{Scanner, Solution};
use std::io::{Read, Write};

#[derive(Default)]
pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scan = Scanner::new(input);

        let n = scan.next_line();
        for _i in 0..n {
            let line = scan.next_line::<String>();
            let r = abbr(line);
            writeln!(output, "{}", r).ok();
        }
    }
}

fn abbr(s: String) -> String {
    let l = s.len();
    if l > 10 {
        return format!(
            "{}{}{}",
            s.chars().next().unwrap(),
            l - 2,
            s.chars().nth(l - 1).unwrap()
        );
    }
    s
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

        test_helper(cases, Solution1::default());
    }
}
