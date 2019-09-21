// https://leetcode-cn.com/problems/climbing-stairs/

// x + 2y = n
pub fn climb_stairs(n: i32) -> i32 {
    let mut res = 0;
    for x in 0..=n {
        for y in 0..=n / 2 {
            if x + 2 * y == n {
                let c = comb(x + y, x);
                // println!("comb({},{})={}", x+y, x, c);
                res += c;
            }
        }
    }
    res
}

// calculate combinator is not so easy...
// https://stackoverflow.com/questions/1838368/calculating-the-amount-of-combinations
fn comb(n: i32, k: i32) -> i32 {
    if k > n {
        return 0;
    }
    let mut r: i64 = 1;
    let mut nn = n;
    for d in 1..=k {
        r *= i64::from(nn);
        nn -= 1;
        r /= i64::from(d);
    }
    r as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(3, climb_stairs(3));
        assert_eq!(2, climb_stairs(2));
        assert_eq!(14930352, climb_stairs(35));
        assert_eq!(1134903170, climb_stairs(44));
    }
}
