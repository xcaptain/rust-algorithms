//! Implement an algorithm to determine if a string has all unique characters. What if you
//! cannot use additional data structures?

use std::collections::HashMap;

/// 使用map来做
pub fn is_unique_with_map(s: String) -> bool {
    let mut m: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let value = m.entry(c).or_insert(0);
        *value += 1;
        if *value >= 2 {
            return false;
        }
    }

    true
}

/// 两遍循环判断是否有重复
pub fn is_unique_no_extra_space(s: String) -> bool {
    let it = s.chars().enumerate();
    for (i, c1) in it.into_iter() {
        let it2 = s.chars().skip(i + 1).enumerate();
        for (_, c2) in it2 {
            if c1 == c2 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        #[derive(Debug, Clone)]
        struct Args {
            s: String,
        }
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            args: Args,
            want: bool,
        }
        let tests: Vec<Test> = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    s: String::from("abc"),
                },
                want: true,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    s: String::from("abcb"),
                },
                want: false,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    s: String::from(""),
                },
                want: true,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    s: String::from("a"),
                },
                want: true,
            },
        ];

        for test in tests.clone() {
            assert_eq!(
                test.want,
                is_unique_with_map(test.args.s),
                "{} fails",
                test.name
            );
        }

        for test in tests.clone() {
            assert_eq!(
                test.want,
                is_unique_no_extra_space(test.args.s),
                "{} fails",
                test.name
            );
        }
    }
}
