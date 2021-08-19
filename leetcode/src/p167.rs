use std::collections::HashMap;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::new();
    for (idx, number) in numbers.iter().enumerate() {
        m.insert(target - *number, (*number, idx as i32));
    }
    for (idx, number) in numbers.iter().enumerate() {
        if m.contains_key(number) {
            let idxi32 = idx as i32 + 1;
            let complement_idx = m[number].1 + 1; // index start from 1 not 0

            if idxi32 < complement_idx {
                return vec![idxi32, complement_idx];
            }
            return vec![complement_idx, idxi32];
        }
    }

    vec![1, 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        #[derive(Debug, Clone)]
        struct Args {
            numbers: Vec<i32>,
            target: i32,
        }
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            args: Args,
            want: Vec<i32>,
        }
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    numbers: vec![2, 7, 11, 15],
                    target: 9,
                },
                want: vec![1, 2],
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    numbers: vec![2, 3, 4],
                    target: 6,
                },
                want: vec![1, 3],
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    numbers: vec![-1, 0],
                    target: -1,
                },
                want: vec![1, 2],
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                two_sum(test.args.numbers, test.args.target),
                "{} fails",
                test.name
            );
        }
    }
}
