/// Fibonacci algorithms
/// f(n+2) = f(n) + f(n+1), n starts from 0
/// 0, 1, 1, 2, 3, 5, 8, 13...

/// the default fib algorithm
pub use fib1 as fib;

/// get the nth fibonacci number using recursive
/// note: when n is large, stack will overflow
pub fn fib1(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib1(n - 1) + fib1(n - 2);
    }
}

/// get the nth fibonacci number using iteration
pub fn fib2(n: usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    if n == 0 {
        return a;
    }
    if n == 1 {
        return b;
    }
    let mut c = 1;
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    return c;
}

/// get the nth fibonacci number using dynamic programming
pub fn fib3(n: usize) -> usize {
    let mut f = vec![0; n + 2];
    f[0] = 0;
    f[1] = 1;
    for i in 2..n + 1 {
        f[i] = f[i - 1] + f[i - 2];
    }
    return f[n];
}

/// get the nth fabonacci number using matrix multiply
/// {{1, 1}, {1, 0}}^n = {{f(n+1), f(n)}, {f(n), f(n-1)}}
/// todo in the future

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib1() {
        assert_eq!(0, fib1(0));
        assert_eq!(3, fib1(4));
        assert_eq!(13, fib1(7));
        assert_eq!(34, fib1(9));
    }

    #[test]
    fn test_fib2() {
        assert_eq!(0, fib2(0));
        assert_eq!(3, fib2(4));
        assert_eq!(34, fib2(9));
    }

    #[test]
    fn test_fib3() {
        assert_eq!(0, fib3(0));
        assert_eq!(3, fib3(4));
        assert_eq!(34, fib3(9));
    }
}
