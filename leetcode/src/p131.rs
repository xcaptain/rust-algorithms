// https://leetcode-cn.com/problems/palindrome-partitioning/

pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut res = vec![];

    backtrack(&s, 0, s.len(), &mut res, vec![]);
    res
}

fn backtrack(s: &str, start: usize, end: usize, res: &mut Vec<Vec<String>>, cur: Vec<String>) {
    if start >= end {
        res.push(cur);
        return;
    }
    for i in start..end {
        if is_palindrome(&s[start..=i]) {
            let mut new_cur = cur.clone();
            new_cur.push(s[start..=i].to_owned());
            backtrack(s, i + 1, end, res, new_cur);
        }
    }
}

fn is_palindrome(s: &str) -> bool {
    let l = s.len();
    for i in 0..=l / 2 {
        let j = l - i - 1;
        if s[i..=i] != s[j..=j] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p131() {
        assert_eq!(
            vec![vec!["a", "a", "b"], vec!["aa", "b"]],
            partition(String::from("aab"))
        );
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, is_palindrome("a"));
        assert_eq!(true, is_palindrome("aa"));
        assert_eq!(true, is_palindrome("aba"));
        assert_eq!(false, is_palindrome("aab"));
    }
}
