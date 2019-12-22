use std::cmp::max;

pub fn integer_break(n: i32) -> i32 {
    let mut dp = vec![0; (n + 2) as usize];
    dp[1] = 1;
    dp[2] = 1;
    for i in 3..=n {
        for j in 1..=i - 1 {
            dp[i as usize] = max(dp[i as usize], max(j * (i - j), j * dp[(i - j) as usize]));
        }
    }
    dp[n as usize]
}

pub fn integer_break_brute_force(n: i32) -> i32 {
    if n == 2 {
        return 1;
    }
    let mut res = -1;
    for i in 1..n {
        res = max(res, max(i * (n - i), i * integer_break(n - i)));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p343() {
        assert_eq!(1, integer_break(2));
        assert_eq!(36, integer_break(10));
    }
}
