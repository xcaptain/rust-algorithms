// https://codeforces.com/problemset/problem/122/A

use crate::Scanner;
use std::io::{Read, Write};

pub fn solution_of_p122a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let num = scanner.next_line::<String>().parse::<usize>().unwrap();
    // because the input is between 1 and 1000, so we can list all the lucky numbers
    let lucky_nums = vec![4, 7, 44, 47, 74, 77, 444, 447, 474, 477, 744, 747, 774, 777];
    if is_almose_lucky(num, lucky_nums) {
        write!(out, "YES").ok();
    } else {
        write!(out, "NO").ok();
    }
}

fn is_almose_lucky(num: usize, arr: Vec<usize>) -> bool {
    if arr.contains(&num) {
        return true;
    }
    for item in &arr {
        if num % (*item) == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p122a() {
        let cases = vec![["47", "YES"], ["16", "YES"], ["78", "NO"]];
        test_helper(cases, solution_of_p122a);
    }
}
