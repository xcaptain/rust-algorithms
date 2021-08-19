//! Given a string, write a function to check if it is a permutation of a palindrome.
//! A palindrome is a word or phrase that is the same forwards and backwards. A permutation
//! is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
//! EXAMPLE
//! Input: Tact Coa
//! Output: True (permutations: "taco cat", "atco eta", etc.)

use std::collections::HashMap;

/// 校验这个字符串是否是一个回文串的排列
pub fn is_palindrome_permutation(s: String) -> bool {
    let s = s.to_ascii_lowercase();
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut l = 0;
    for c in s.chars() {
        if c == ' ' {
            continue;
        }
        let value = map.entry(c).or_insert(0);
        *value += 1;
        l += 1;
    }
    if l % 2 == 0 {
        // 字符串长度为偶数
        for (_k, v) in map {
            if v % 2 == 1 {
                // 如果有不匹配的，直接返回false
                return false;
            }
        }
    } else {
        let mut odd_num = 0;
        for (_k, v) in map {
            if v % 2 == 1 {
                odd_num += 1;
            }
        }
        if odd_num != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_permutation() {
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
                    s: String::from("T a c t Coa"),
                },
                want: true,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    s: String::from("jhsabckuj ahjsbckj"),
                },
                want: true,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    s: String::from("Able was I ere I saw Elba"),
                },
                want: true,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    s: String::from("Random Words"),
                },
                want: false,
            },
            Test {
                name: String::from("case5"),
                args: Args {
                    s: String::from("So patient a nurse to nurse a patient so"),
                },
                want: false,
            },
        ];

        {
            for test in tests {
                assert_eq!(
                    test.want,
                    is_palindrome_permutation(test.args.s),
                    "{} fails",
                    test.name
                );
            }
        }
    }
}
