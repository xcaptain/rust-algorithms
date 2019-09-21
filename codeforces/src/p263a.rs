// https://codeforces.com/problemset/problem/263/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p263a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let mut mat: Vec<Vec<usize>> = vec![];
    for _i in 0..5 {
        let arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        mat.push(arr);
    }

    // find the cordinate of the 1 in the matrix
    let mut x = 0;
    let mut y = 0;
    for (i, mat_item) in mat.iter().enumerate().take(5) {
        for (j, elem) in mat_item.iter().enumerate().take(5) {
            if elem == &1 {
                x = i;
                y = j;
            }
        }
    }
    let mut res = 0;
    if x > 2 {
        res += x - 2;
    } else {
        res += 2 - x;
    }
    if y > 2 {
        res += y - 2;
    } else {
        res += 2 - y;
    }
    write!(out, "{}", res).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p263a() {
        let cases = vec![
            [
                "0 0 0 0 0
0 0 0 0 1
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
",
                "3",
            ],
            [
                "0 0 0 0 0
0 0 0 0 0
0 1 0 0 0
0 0 0 0 0
0 0 0 0 0
",
                "1",
            ],
        ];

        test_helper(cases, solution_of_p263a);
    }
}
