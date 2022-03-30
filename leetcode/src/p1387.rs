use std::collections::HashMap;

// https://leetcode-cn.com/problems/sort-integers-by-the-power-value/

pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut num_and_steps: Vec<(i32, i32)> = vec![(0, 0); (hi - lo + 1) as usize];
    let mut step_and_counts: HashMap<i32, i32> = HashMap::new();
    for (idx, item) in (lo..=hi).enumerate() {
        let step = get_steps(item);
        num_and_steps[idx] = (item, step);
        let counter = step_and_counts.entry(step).or_insert(0);
        *counter += 1;
    }

    num_and_steps.sort_by(|a, b| a.1.cmp(&b.1));

    // 手动排序重复部分
    let l = num_and_steps.len();
    let mut i = 0;
    while i < l {
        let next_i = step_and_counts[&num_and_steps[i].1] as usize + i;
        let mut arr = vec![(0, 0); next_i - i];
        arr.copy_from_slice(&num_and_steps[i..next_i]);
        arr.sort_by(|a, b| a.0.cmp(&b.0));
        num_and_steps[i..next_i].copy_from_slice(&arr);
        i = next_i;
    }

    num_and_steps[k as usize - 1].0
}

pub fn get_kth_1(lo: i32, hi: i32, k: i32) -> i32 {
    let mut num_and_steps: Vec<(i32, i32)> = vec![(0, 0); (hi - lo + 1) as usize];
    let mut step_and_counts: HashMap<i32, i32> = HashMap::new();
    for (idx, item) in (lo..=hi).enumerate() {
        let step = get_steps(item);
        num_and_steps[idx] = (item, step);
        let counter = step_and_counts.entry(step).or_insert(0);
        *counter += 1;
    }

    num_and_steps.sort_by(|a, b| {
        if a.1 != b.1 {
            return a.1.cmp(&b.1);
        }
        a.0.cmp(&b.0)
    });

    num_and_steps[k as usize - 1].0
}

fn get_steps(mut n: i32) -> i32 {
    let mut res = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
            res += 1;
        } else {
            n = n * 3 + 1;
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_kth() {
        #[derive(Debug, Clone)]
        struct Args {
            lo: i32,
            hi: i32,
            k: i32,
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
                args: Args {
                    lo: 12,
                    hi: 15,
                    k: 2,
                },
                want: 13,
            },
            Test {
                name: String::from("case2"),
                args: Args { lo: 1, hi: 1, k: 1 },
                want: 1,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    lo: 7,
                    hi: 11,
                    k: 4,
                },
                want: 7,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    lo: 10,
                    hi: 20,
                    k: 5,
                },
                want: 13,
            },
            Test {
                name: String::from("case5"),
                args: Args {
                    lo: 1,
                    hi: 1000,
                    k: 777,
                },
                want: 570,
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                get_kth(test.args.lo, test.args.hi, test.args.k),
                "{} fails",
                test.name
            );
        }
    }

    #[test]
    fn test_get_kth_1() {
        #[derive(Debug, Clone)]
        struct Args {
            lo: i32,
            hi: i32,
            k: i32,
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
                args: Args {
                    lo: 12,
                    hi: 15,
                    k: 2,
                },
                want: 13,
            },
            Test {
                name: String::from("case2"),
                args: Args { lo: 1, hi: 1, k: 1 },
                want: 1,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    lo: 7,
                    hi: 11,
                    k: 4,
                },
                want: 7,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    lo: 10,
                    hi: 20,
                    k: 5,
                },
                want: 13,
            },
            Test {
                name: String::from("case5"),
                args: Args {
                    lo: 1,
                    hi: 1000,
                    k: 777,
                },
                want: 570,
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                get_kth_1(test.args.lo, test.args.hi, test.args.k),
                "{} fails",
                test.name
            );
        }
    }
}
