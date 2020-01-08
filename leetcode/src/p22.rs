// https://leetcode-cn.com/problems/generate-parentheses/

// ()
// ()(), (())
// ()()(), (()()), (())(), ((()))

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    backtrack(&mut res, String::from(""), 0, 0, n as usize);
    res
}

fn backtrack(res: &mut Vec<String>, cur: String, open: usize, close: usize, max: usize) {
    if cur.len() == 2 * max {
        res.push(cur);
        return;
    }
    if open < max {
        backtrack(res, format!("{}(", cur), open + 1, close, max);
    }
    if close < open {
        backtrack(res, format!("{})", cur), open, close + 1, max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p22() {
        assert_eq!(
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"],
            generate_parenthesis(3)
        );

        assert_eq!(
            vec![
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ],
            generate_parenthesis(4)
        );
    }
}
