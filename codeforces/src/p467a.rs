// https://codeforces.com/problemset/problem/467/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p467a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let n = scanner.next_line::<String>().parse::<usize>().unwrap();
    let mut res = 0;
    for _i in 0..n {
        let arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        let p = arr[0];
        let q = arr[1];
        if q - p >= 2 {
            res += 1;
        }
    }
    write!(out, "{}", res).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p467a() {
        let cases = vec![
            [
                "3
1 1
2 2
3 3",
                "0",
            ],
            [
                "3
1 10
0 10
10 10",
                "2",
            ],
        ];
        test_helper(cases, solution_of_p467a);
    }
}
