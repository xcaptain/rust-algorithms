// https://codeforces.com/problemset/problem/69/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p69a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let n = scanner.next_line::<String>().parse::<usize>().unwrap();
    let mut mat: Vec<Vec<isize>> = vec![];
    for _i in 0..n {
        let arr = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<isize>().unwrap())
            .collect();
        mat.push(arr);
    }
    if check_is_equilibrium(mat) {
        write!(out, "YES").ok();
    } else {
        write!(out, "NO").ok();
    }
}

#[allow(clippy::needless_range_loop)]
fn check_is_equilibrium(mat: Vec<Vec<isize>>) -> bool {
    let n = mat.len();
    for j in 0..3 {
        let mut res = 0;
        for i in 0..n {
            res += mat[i][j];
        }
        if res != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p69a() {
        let cases = vec![
            [
                "3
4 1 7
-2 4 -1
1 -5 -3",
                "NO",
            ],
            [
                "3
3 -1 7
-5 2 -4
2 -1 -3",
                "YES",
            ],
        ];

        test_helper(cases, solution_of_p69a);
    }
}
