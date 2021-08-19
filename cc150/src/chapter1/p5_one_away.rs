//! There are three types of edits that can be performed on strings: insert a character,
//! remove a character, or replace a character. Given two strings, write a function to check if they are
//! one edit (or zero edits) away.
//! EXAMPLE
//! pale, ple -> true
//! pales, pale -> true
//! pale, bale -> true
//! pale, bake -> false

/// 判断1个字符串是否仅用0或1次变换就能得到另一个字符串，
/// 变换包括，插入，删除，替换一个字符
pub fn one_away(s1: String, s2: String) -> bool {
    let l1 = s1.len();
    let l2 = s2.len();
    if l1 == l2 {
        return one_edit_edit(s1, s2);
    } else if l1 >= l2 && l1 - l2 == 1 {
        return one_edit_insert(s2, s1);
    } else if l2 >= l1 && l2 - l1 == 1 {
        return one_edit_insert(s1, s2);
    }
    false
}

/// test if s1 can become s2 in 1 edit op,
/// s1 and s2 have the same length
fn one_edit_edit(s1: String, s2: String) -> bool {
    let mut edited = false;
    for i in 0..s1.len() {
        if s1[i..=i] != s2[i..=i] {
            if edited == true {
                return false;
            }
            edited = true;
        }
    }
    true
}

/// test if s1 can become s2 using 1 insert op,
/// s1 is shorter and s2 is longer and s1.len() + 1 == s2.len()
fn one_edit_insert(shorter: String, longer: String) -> bool {
    let mut edited = false;
    let mut i = 0;
    let mut j = 0;
    while i < shorter.len() && j < longer.len() {
        if shorter[i..=i] != longer[j..=j] {
            if edited == true {
                return false;
            }
            j += 1;
            edited = true;
        } else {
            i += 1;
            j += 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_away() {
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
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    s1: String::from("pale"),
                    s2: String::from("ple"),
                },
                want: true,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    s1: String::from("pales"),
                    s2: String::from("pale"),
                },
                want: true,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    s1: String::from("pale"),
                    s2: String::from("bale"),
                },
                want: true,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    s1: String::from("pal"),
                    s2: String::from("psa"),
                },
                want: false,
            },
        ];

        {
            for test in tests {
                assert_eq!(
                    test.want,
                    one_away(test.args.s1, test.args.s2),
                    "{} fails",
                    test.name
                );
            }
        }
    }
}
