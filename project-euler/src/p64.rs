pub fn solution_of_p64() -> usize {
    let mut num = 0;
    for s in 2..10_000 {
        let arr = continue_fraction_seq(s);
        if arr.len() % 2 == 0 {
            num += 1;
        }
    }
    num
}

/// 参考：https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Continued_fraction_expansion
/// a(n+1) = (a0 + m(n+1))/d(n+1)
/// a0 = sqrt(s)
/// d0 = 1
/// m0 = 0
/// d(n+1) = (s - m(n+1)^2)/d(n)
/// m(n+1) = d(n)*a(n) - m(n)
pub fn continue_fraction_seq(s: usize) -> Vec<usize> {
    let a0 = (s as f64).sqrt() as usize;
    let mut arr = vec![a0];
    if a0 * a0 == s {
        // 如果s是完全平方数，则没必要做连分数变换
        return arr;
    }
    let mut a = a0;
    let mut d = 1;
    let mut m = 0;
    let mut m_arr = vec![];
    let mut d_arr = vec![];

    loop {
        m = d * a - m;
        d = (s - m * m) / d;
        a = (a0 + m) / d;
        if let Some(mpos) = m_arr.iter().position(|&r| r == m) {
            if let Some(dpos) = d_arr.iter().position(|&r| r == d) {
                if mpos == dpos {
                    break;
                }
            }
        }
        arr.push(a);
        m_arr.push(m);
        d_arr.push(d);
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p64() {
        assert_eq!(1322, solution_of_p64());
    }

    #[test]
    fn test_continue_fraction_seq() {
        assert_eq!(vec![4, 1, 3, 1, 8], continue_fraction_seq(23));
        assert_eq!(vec![1, 2], continue_fraction_seq(2));
        assert_eq!(vec![2], continue_fraction_seq(4));
    }
}
