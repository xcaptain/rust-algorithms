// http://mathworld.wolfram.com/PartitionFunctionP.html
// must first know the recursion function to compute,
// the base is math
pub fn solution_of_p76() -> usize {
    return part(100);
}

fn part(n: usize) -> usize {
    let mut s = 0;
    for k in 1..n {
        s += p(n, k);
    }
    return s;
}

fn p(n: usize, k: usize) -> usize {
    if k == 1 {
        return 1;
    }
    if k > n {
        return 0;
    }
    return p(n - 1, k - 1) + p(n - k, k);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p76() {
        assert_eq!(190569291, solution_of_p76());
    }

    #[test]
    fn test_sum_num() {
        assert_eq!(4, part(4));
        assert_eq!(6, part(5));
        assert_eq!(10, part(6));
    }
}
