// https://codeforces.com/problemset/problem/266/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p266a(input: &mut Read, out: &mut Write) {
    let mut scanner = Scanner::new(input);
    let _n = scanner.next::<String>().parse::<usize>().unwrap();
    let arr: Vec<char> = scanner.next::<String>().chars().collect();
    let res = min_move_ways(arr);
    write!(out, "{}\n", res).ok();
}

fn min_move_ways(mut arr: Vec<char>) -> usize {
    if arr.len() <= 1 {
        return 0;
    } else if arr.len() == 2 {
        if arr[0] == arr[1] {
            return 1;
        }
        return 0;
    }
    let mut i = 0;
    let mut res = 0;
    while i < arr.len() - 2 {
        if arr[i] == arr[i + 1] || arr[i + 1] == arr[i + 2] {
            arr.remove(i + 1);
            res += 1;
        } else {
            i += 1;
        }
    }
    if arr.len() <= 1 {
        return res;
    } else if arr.len() == 2 {
        if arr[0] == arr[1] {
            return res + 1;
        }
        return res;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p266a() {
        let cases = vec![
            [
                "3
RRG", "1",
            ],
            [
                "5
RRRRR
", "4",
            ],
            [
                "4
BRBG
", "0",
            ],
            [
                "5
BRBGG
", "1",
            ],
        ];
        test_helper(cases, solution_of_p266a);
    }
}
