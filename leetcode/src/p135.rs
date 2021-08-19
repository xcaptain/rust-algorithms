// https://leetcode-cn.com/problems/candy/

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut left = vec![1; ratings.len()];
    let l = ratings.len();
    for i in 0..l {
        if i > 0 && ratings[i] > ratings[i - 1] {
            left[i] = left[i - 1] + 1;
        }
    }
    let mut right = 0;
    let mut ret = 0;
    for i in (0..l).rev() {
        if i < l - 1 && ratings[i] > ratings[i + 1] {
            right += 1;
        } else {
            right = 1;
        }
        ret += left[i].max(right);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy() {
        #[derive(Debug, Clone)]
        struct Args {
            ratings: Vec<i32>,
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
                    ratings: vec![1, 0, 2],
                },
                want: 5,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    ratings: vec![1, 2, 2],
                },
                want: 4,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    ratings: vec![1, 3, 2, 2, 1],
                },
                want: 7,
            },
        ];

        {
            for test in tests {
                assert_eq!(test.want, candy(test.args.ratings), "{} fails", test.name);
            }
        }
    }
}
