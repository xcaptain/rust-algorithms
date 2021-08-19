//! Implement a method to perform basic string compression using the counts
//! of repeated characters. For example, the string aabcccccaaa would become a2blc5a3. If the
//! "compressed" string would not become smaller than the original string, your method should return
//! the original string. You can assume the string has only uppercase and lowercase letters (a - z).

pub fn compress(s1: String) -> String {
    let mut res = String::new();
    let mut curr_char = '0'; // a special sign, because s1 only contains a-zA-Z
    let mut curr_num = 0;

    for c in s1.chars() {
        if curr_char == '0' {
            curr_char = c;
            curr_num = 1;
            continue;
        }
        if c == curr_char {
            curr_num += 1;
        } else {
            if curr_num > 0 {
                res.push_str(format!("{}", curr_char).as_str());
                res.push_str(format!("{}", curr_num).as_str());
            }
            curr_char = c;
            curr_num = 1;
        }
    }
    if curr_num > 0 && curr_char != '0' {
        res.push_str(format!("{}", curr_char).as_str());
        res.push_str(format!("{}", curr_num).as_str());
    }

    if res.len() >= s1.len() {
        return s1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        struct Args {
            s: String,
        }
        struct Test {
            name: String,
            args: Args,
            want: String,
        }
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    s: String::from("aabcccccaaa"),
                },
                want: String::from("a2b1c5a3"),
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    s: String::from("abcdef"),
                },
                want: String::from("abcdef"),
            },
        ];
        for test in tests {
            assert_eq!(test.want, compress(test.args.s), "{} fails", test.name);
        }
    }
}
