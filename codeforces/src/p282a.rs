// https://codeforces.com/problemset/problem/282/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p282a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let n = scanner.next_line::<String>().parse::<usize>().unwrap();
    let mut val = 0;
    for _i in 0..n {
        let statement = scanner.next_line::<String>();
        if statement.contains("++") {
            val += 1;
        } else if statement.contains("--") {
            val -= 1;
        } else {
            panic!("unimplemented statement");
        }
    }
    write!(out, "{}", val).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p282a() {
        let cases = vec![
            [
                "1
++X", "1",
            ],
            [
                "2
X++
--X",
                "0",
            ],
        ];

        test_helper(cases, solution_of_p282a);
    }
}
