/// get triangular number whose divisors have over divisor_num
pub fn solution(divisor_num: usize) -> usize {
    let mut i = 1;
    loop {
        let tri_num = triangular_number(i);
        if get_divisors(tri_num) > divisor_num {
            return tri_num;
        }
        i += 1;
    }
}

/// use square root to reduce recompute
pub fn get_divisors(n: usize) -> usize {
    let mut num = 0;
    let mid = (n as f64).sqrt() as usize + 1;
    for i in 1..=mid {
        if n % i == 0 {
            num += 2;
        }
    }
    num
}

pub fn triangular_number(n: usize) -> usize {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(76576500, solution(500));
    }

    #[test]
    fn test_get_divisors() {
        assert_eq!(4, get_divisors(21));
        assert_eq!(6, get_divisors(28));
    }
}
