// https://codeforces.com/problemset/problem/617/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_for_cf(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let x = scanner.next_line::<i32>();
    let steps = solution(x);

    write!(out, "{}", steps).ok();
}

fn solution(x: i32) -> i32 {
    if x <= 5 {
        return 1;
    }
    if x % 5 == 0 {
        return x / 5;
    }
    x / 5 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_for_cf() {
        let cases = vec![["5", "1"], ["12", "3"], ["1000000", "200000"]];
        test_helper(cases, solution_for_cf);
    }
}
