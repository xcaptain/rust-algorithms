// https://codeforces.com/problemset/problem/116/A

use crate::Scanner;
use std::cmp::max;
use std::io::{Read, Write};

pub fn solution_of_p116a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let n = scanner.next_line::<String>().parse::<usize>().unwrap();
    let mut res = 0;
    let mut current_num = 0;
    for _i in 0..n {
        let arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        let a = arr[0]; // people leave the train
        current_num -= a;
        let b = arr[1]; // people go into the train
        current_num += b;
        res = max(res, current_num);
    }
    writeln!(out, "{}", res).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p116a() {
        let cases = vec![[
            "4
0 3
2 5
4 2
4 0",
            "6",
        ]];
        test_helper(cases, solution_of_p116a);
    }
}
