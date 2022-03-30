// https://codeforces.com/problemset/problem/236/A

use crate::Scanner;
use std::collections::HashSet;
use std::io::{Read, Write};

// it's simple to use hashset, because it doesn't contain duplicate elements
// the trick that convert Vec<char> to HashSet<char> is interesting
pub fn solution_of_p236a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let uniq_arr: HashSet<char> = scanner.next_line::<String>().chars().collect();
    let res = uniq_arr.len();
    if res % 2 == 1 {
        write!(out, "IGNORE HIM!").ok();
    } else {
        write!(out, "CHAT WITH HER!").ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p236a() {
        let cases = vec![
            ["wjmzbmr", "CHAT WITH HER!"],
            ["xiaodao", "IGNORE HIM!"],
            ["sevenkplus", "CHAT WITH HER!"],
        ];
        test_helper(cases, solution_of_p236a);
    }
}
