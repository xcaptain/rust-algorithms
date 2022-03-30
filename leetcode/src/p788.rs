// https://leetcode-cn.com/problems/rotated-digits/submissions/

pub fn rotated_digits(n: i32) -> i32 {
    let mut res = 0;
    for i in 1..=n {
        if is_good(i) {
            res += 1;
        }
    }
    res
}

fn is_good(num: i32) -> bool {
    if num < 10 {
        if num == 2 || num == 5 || num == 6 || num == 9 {
            return true;
        }
        return false;
    }
    let new_num = to_good(num);
    if new_num == -1 {
        return false;
    }
    num != new_num
}

fn to_good(mut num: i32) -> i32 {
    let mut res = 0;
    let mut i = 0;
    while num > 0 {
        let q = num % 10;
        let new_q: i32;
        if q == 0 {
            new_q = 0;
        } else if q == 1 {
            new_q = 1;
        } else if q == 2 {
            new_q = 5;
        } else if q == 5 {
            new_q = 2;
        } else if q == 8 {
            new_q = 8;
        } else if q == 6 {
            new_q = 9;
        } else if q == 9 {
            new_q = 6;
        } else {
            return -1;
        }
        num = (num - q) / 10;
        res += 10_i32.pow(i) * new_q;
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotated_digits() {
        #[derive(Debug, Clone)]
        struct Args {
            n: i32,
        }
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            args: Args,
            want: i32,
        }
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args { n: 10 },
                want: 4,
            },
            // 2, 5, 6, 9, 12, 15, 16, 19, 20
            Test {
                name: String::from("case2"),
                args: Args { n: 20 },
                want: 9,
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                rotated_digits(test.args.n),
                "{} fails",
                test.name
            );
        }
    }
}
