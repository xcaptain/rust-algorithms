// https://codeforces.com/problemset/problem/339/A

use crate::Scanner;
use std::io::{Read, Write};

// the input string only contains "+" and 1 2 3, so can sort the array
pub fn solution_of_p339a(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let line1 = scanner.next_line::<String>();
    let mut arr: Vec<usize> = line1
        .split('+')
        .map(|e| e.parse::<usize>().unwrap())
        .collect();
    arr.sort_unstable(); // asc

    let arr2: Vec<String> = arr.into_iter().map(|e: usize| e.to_string()).collect();
    let res = arr2.join("+");
    write!(out, "{}", res).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_solution_of_p339a() {
        let cases = vec![["3+2+1", "1+2+3"], ["1+1+3+1+3", "1+1+1+3+3"], ["2", "2"]];
        test_helper(cases, solution_of_p339a);
    }
}
