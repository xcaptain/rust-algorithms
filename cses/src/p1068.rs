/// Consider an algorithm that takes as input a positive integer n. If n is even, the algorithm divides it by two, and if n is odd, the algorithm multiplies it by three and adds one. The algorithm repeats this, until n is one. For example, the sequence for n=3 is as follows:
/// 3→10→5→16→8→4→2→1

/// Your task is to simulate the execution of the algorithm for a given value of n.

/// Input

/// The only input line contains an integer n.

/// Output

/// Print a line that contains all values of n during the algorithm.

/// Constraints
/// 1≤n≤106
/// Example

/// Input:
/// 3

/// Output:
/// 3 10 5 16 8 4 2 1
use crate::Scanner;
use std::io::{Read, Write};

pub fn solve(input: &mut dyn Read, out: &mut dyn Write) {
    let mut scanner = Scanner::new(input);
    let mut n = scanner.next_line::<usize>();
    let _ = write!(out, "{} ", n);
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        let _ = write!(out, "{} ", n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn test() {
        let cases = vec![["3", "3 10 5 16 8 4 2 1"]];

        test_helper(cases, solve);
    }
}
