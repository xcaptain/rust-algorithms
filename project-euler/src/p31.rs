/// 获取可能的硬币表示数
/// n = 1*a + 2*b + 5*c + 10*d + 20*e + 50*f + 100*g + 200*h
pub fn count_coins(n: usize) -> usize {
    let mut num = 0;
    for h in 0..=1 {
        for g in 0..=2 {
            for f in 0..=4 {
                for e in 0..=10 {
                    for d in 0..=20 {
                        for c in 0..=40 {
                            for b in 0..=100 {
                                for a in 0..=200 {
                                    if a + 2 * b
                                        + 5 * c
                                        + 10 * d
                                        + 20 * e
                                        + 50 * f
                                        + 100 * g
                                        + 200 * h
                                        == n
                                    {
                                        num += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    num
}

pub fn count_methods(money: isize, max_coin: usize) -> usize {
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    // let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut s = 0;
    for (i, _) in coins.iter().enumerate().skip(max_coin) {
        let remain: isize = money - coins[i];
        if remain == 0 {
            s += 1;
        }
        if remain > 0 {
            s += count_methods(remain, i);
        }
    }
    s
}

pub fn f(money: isize, coins: &[isize]) -> isize {
    let l = coins.len();
    if l == 0 {
        return 0;
    }
    if money == 0 {
        return 1;
    }
    if money < 0 {
        return 0;
    }
    f(money - coins[0], coins) + f(money, &coins[1..l])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_pounds() {
        // assert_eq!(73682, count_coins(200));
        assert_eq!(73682, count_methods(200, 0));
        assert_eq!(1, count_methods(1, 0));
        assert_eq!(2, count_methods(2, 0));
        assert_eq!(2, count_methods(3, 0));

        assert_eq!(73682, f(200, &[200, 100, 50, 20, 10, 5, 2, 1]));
    }
}
