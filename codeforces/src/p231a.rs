// https://codeforces.com/problemset/problem/231/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p231a(input: &mut Read, out: &mut Write) {
    let mut scanner = Scanner::new(input);
    let n = scanner.next::<String>().parse::<usize>().unwrap();
    let mut res = 0;
    for _i in 0..n {
        let arr: Vec<usize> = scanner
            .next::<String>()
            .split(' ')
            .map(|e| {
                return e.parse::<usize>().unwrap();
            })
            .collect();
        let mut arr_n = 0;
        for item in &arr {
            if item == &1 {
                arr_n += 1;
            }
        }
        if arr_n >= 2 {
            res += 1;
        }
    }
    write!(out, "{}\n", res).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p231a() {
        let cases = vec![
            [
                "3
1 1 0
1 1 1
1 0 0",
                "2",
            ],
            [
                "2
1 0 0
0 1 1",
                "1",
            ],
        ];
        test_helper(cases, solution_of_p231a);
    }
}
