// http://mathworld.wolfram.com/PartitionFunctionP.html
// must first know the recursion function to compute,
// the base is math

use std::collections::HashMap;

pub fn solution_of_p76() -> usize {
    part1(100)
}

// simple recursion solution
fn part1(n: usize) -> usize {
    let mut s = 0;
    for k in 1..n {
        s += p1(n, k);
    }
    s
}

// dynamic programming, using p_cache to store some computed values
// which can reduce needless recursion
#[allow(dead_code)]
fn part2(n: usize) -> usize {
    let mut s = 0;
    let mut p_cache = HashMap::new();
    for k in 1..n {
        if let Some(p_val) = p_cache.get(&(n, k)) {
            s += p_val;
        } else {
            let n_val = p1(n, k);
            s += n_val;
            p_cache.insert((n, k), n_val);
        }
    }
    s
}

fn p1(n: usize, k: usize) -> usize {
    if k == 1 {
        return 1;
    }
    if k > n {
        return 0;
    }
    p1(n - 1, k - 1) + p1(n - k, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p76() {
        assert_eq!(190569291, solution_of_p76());
        assert_eq!(190569291, part2(100));
    }

    #[test]
    fn test_sum_num() {
        assert_eq!(4, part1(4));
        assert_eq!(6, part1(5));
        assert_eq!(10, part1(6));

        assert_eq!(4, part2(4));
        assert_eq!(6, part2(5));
        assert_eq!(10, part2(6));
    }
}
