// https://codeforces.com/problemset/problem/158/A
use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p158a(input: &mut Read, out: &mut Write) {
    let mut scanner = Scanner::new(input);

    let line1 = scanner.next::<String>();
    let line1_arr: Vec<usize> = line1
        .split(" ")
        .map(|s| {
            return s.parse::<usize>().unwrap();
        })
        .collect();
    let k = line1_arr[1];

    let line2 = scanner.next::<String>();
    let line2_arr: Vec<usize> = line2
        .split(" ")
        .map(|s| {
            return s.parse::<usize>().unwrap();
        })
        .collect();

    let mut res = 0;
    for item in &line2_arr {
        if item >= &line2_arr[k - 1] && item > &0 {
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
    fn test_solution_of_p158a() {
        let cases = vec![
            [
                "8 5
10 9 8 7 7 7 5 5
",
                "6",
            ],
            [
                "4 2
0 0 0 0
",
                "0",
            ],
            [
                "5 5
1 1 1 1 1
",
                "5",
            ],
        ];

        test_helper(cases, solution_of_p158a);
    }
}
