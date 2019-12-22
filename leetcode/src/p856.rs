pub fn score_of_parentheses(s: String) -> i32 {
    helper(&s[..], 0, s.len()) as i32
}

fn helper(s: &str, i: usize, j: usize) -> usize {
    let mut bal = 0;
    let mut ans = 0;
    let mut i = i;
    for k in i..j {
        if &s[k..k + 1] == "(" {
            bal += 1;
        } else {
            bal += -1;
        }
        if bal == 0 {
            if k - i == 1 {
                ans += 1;
            } else {
                ans += 2 * helper(s, i + 1, k);
            }
            i = k + 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p856() {
        assert_eq!(1, score_of_parentheses(String::from("()")));
        assert_eq!(2, score_of_parentheses(String::from("(())")));
        assert_eq!(2, score_of_parentheses(String::from("()()")));
        assert_eq!(6, score_of_parentheses(String::from("(()(()))")));
    }
}
