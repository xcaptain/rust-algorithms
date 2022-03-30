//! Given two strings, write a method to decide if one is a permutation of the other.

use std::{collections::HashMap, iter::FromIterator};

/// 比较每个字符出现的次数来判断是否能调整顺序得到另一个字符串
pub fn is_permutation_character_count(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut m1: HashMap<char, i32> = HashMap::new();
    for c in s1.chars() {
        let value = m1.entry(c).or_insert(0);
        *value += 1;
    }

    let mut m2: HashMap<char, i32> = HashMap::new();
    for c in s2.chars() {
        let value = m2.entry(c).or_insert(0);
        *value += 1;
    }

    for (k1, v1) in m1 {
        if let Some(v2) = m2.get(&k1) {
            if v1 != *v2 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

/// 将2个字符串排序后再比较
pub fn is_permutation_sort(s1: String, s2: String) -> bool {
    let mut a1 = s1.chars().collect::<Vec<char>>();
    a1.sort_unstable();
    let s1 = String::from_iter(a1);

    let mut a2 = s2.chars().collect::<Vec<char>>();
    a2.sort_unstable();
    let s2 = String::from_iter(a2);

    s1 == s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        #[derive(Debug, Clone)]
        struct Args {
            s1: String,
            s2: String,
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
                    s1: String::from("abc"),
                    s2: String::from("cab"),
                },
                want: true,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    s1: String::from("abc"),
                    s2: String::from("ab"),
                },
                want: false,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    s1: String::from("abb"),
                    s2: String::from("ab"),
                },
                want: false,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    s1: String::from("abc"),
                    s2: String::from("abd"),
                },
                want: false,
            },
        ];

        {
            for test in tests.clone() {
                assert_eq!(
                    test.want,
                    is_permutation_character_count(test.args.s1, test.args.s2),
                    "{} fails",
                    test.name
                );
            }
        }

        {
            for test in tests {
                assert_eq!(
                    test.want,
                    is_permutation_sort(test.args.s1, test.args.s2),
                    "{} fails",
                    test.name
                );
            }
        }
    }
}
