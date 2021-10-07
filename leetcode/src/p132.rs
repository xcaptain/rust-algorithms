// https://leetcode-cn.com/problems/palindrome-partitioning-ii/

pub fn min_cut(s: String) -> i32 {
    let l = s.len();
    let mut g = vec![vec![true; l]; l];
    for i in (0..l).rev() {
        for j in i + 1..l {
            g[i][j] = (s[i..=i] == s[j..=j]) && g[i + 1][j - 1];
        }
    }

    let mut f = vec![i32::MAX; l];
    for i in 0..l {
        if g[0][i] {
            f[i] = 0;
        } else {
            for j in 0..i {
                if g[j + 1][i] {
                    f[i] = f[i].min(f[j] + 1);
                }
            }
        }
    }
    f[l - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cut() {
        #[derive(Debug, Clone)]
        struct Args {
            s: String,
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
                    s: String::from("aab"),
                },
                want: 1,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    s: String::from("a"),
                },
                want: 0,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    s: String::from("ab"),
                },
                want: 1,
            },
        ];

        for test in tests {
            assert_eq!(test.want, min_cut(test.args.s), "{} fails", test.name);
        }
        // assert_eq!(1, min_cut(String::from("aab")));
    }
}
