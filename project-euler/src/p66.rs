use crate::p64::continue_fraction_seq;
use crate::p65::convergent;
use num_bigint::BigInt;

pub fn solution_of_p66() -> usize {
    return get_min_d(1000);
}

fn get_min_d(n: usize) -> usize {
    let mut largest = BigInt::from(1);
    let mut min_d = 0;
    for d in 2..n {
        let x = solve(d);
        if x > largest {
            largest = x;
            min_d = d;
        }
    }
    return min_d;
}

/// 解佩尔方程的方法
/// 见：https://en.wikipedia.org/wiki/Pell%27s_equation
fn solve(d: usize) -> BigInt {
    let square_root = (d as f64).sqrt() as usize;
    if square_root * square_root == d {
        // 完全平方数则无解
        return BigInt::from(0);
    }
    let mut arr = continue_fraction_seq(d); // 目前arr只包含重复部位
    let arr_c = arr.clone();
    let dup_arr = &arr_c[1..];
    for _ in 1..10 {
        arr.extend(dup_arr); // 不知道要计算到何时，先复制10份
    }
    let mut n = 1;
    loop {
        let f = convergent(n, arr.clone());
        let x = f.numerator.clone();
        let y = f.denominator.clone();
        let result = f.numerator.clone() * x.clone() - d * f.denominator.clone() * y.clone();
        if result == BigInt::from(1) {
            return x;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p66() {
        assert_eq!(661, solution_of_p66());
    }

    #[test]
    fn test_solve() {
        assert_eq!(BigInt::from(8), solve(7));
        assert_eq!(BigInt::from(649), solve(13));
    }

    #[test]
    fn test_get_min_d() {
        assert_eq!(5, get_min_d(10));
    }
}
