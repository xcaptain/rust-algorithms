pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let l = nums.len();
    if l < 2 {
        return false;
    }
    for i in 0..l - 1 {
        for j in i + 1..l {
            if (j - i) as i32 <= k && (nums[i] as i64 - nums[j] as i64).abs() <= t as i64 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_almost_duplicate() {
        #[derive(Debug, Clone)]
        struct Args {
            nums: Vec<i32>,
            k: i32,
            t: i32,
        }
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            args: Args,
            want: bool,
        }
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    nums: vec![1, 2, 3, 1],
                    k: 3,
                    t: 0,
                },
                want: true,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    nums: vec![1, 0, 1, 1],
                    k: 1,
                    t: 2,
                },
                want: true,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    nums: vec![1, 5, 9, 1, 5, 9],
                    k: 2,
                    t: 3,
                },
                want: false,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    nums: vec![],
                    k: 0,
                    t: 0,
                },
                want: false,
            },
            Test {
                name: String::from("case5"),
                args: Args {
                    nums: vec![-2147483648, 2147483647],
                    k: 1,
                    t: 1,
                },
                want: false,
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                contains_nearby_almost_duplicate(test.args.nums, test.args.k, test.args.t),
                "{} fails",
                test.name
            );
        }
    }
}
