// https://leetcode-cn.com/problems/word-break/

use std::collections::VecDeque;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut q = VecDeque::new();
    let mut visited = vec![false; s.len()];
    q.push_back(0);
    while !q.is_empty() {
        let start = q.pop_front().unwrap();
        if !visited[start] {
            for i in start + 1..=s.len() {
                if word_dict.contains(&s[start..i].to_owned()) {
                    q.push_back(i);
                    if i == s.len() {
                        return true;
                    }
                }
            }
            visited[start] = true;
        }
    }
    false
}

pub fn word_break_backtrack(s: String, word_dict: Vec<String>) -> bool {
    let mut res = vec![];

    backtrack(&s, 0, s.len(), &word_dict, &mut res, vec![]);

    !res.is_empty()
}

// OOT
fn backtrack(
    s: &str,
    start: usize,
    end: usize,
    word_dict: &[String],
    res: &mut Vec<Vec<String>>,
    cur: Vec<String>,
) {
    if start == end {
        res.push(cur);
        return;
    }
    for i in start..end {
        if word_dict.contains(&s[start..=i].to_owned()) {
            let mut new_cur = cur.clone();
            new_cur.push(s[start..=i].to_owned());
            backtrack(s, i + 1, end, word_dict, res, new_cur);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p139() {
        assert_eq!(
            true,
            word_break(
                String::from("leetcode"),
                vec![String::from("leet"), String::from("code")]
            )
        );
        assert_eq!(
            true,
            word_break(
                String::from("applepenapple"),
                vec![String::from("apple"), String::from("pen")]
            )
        );
        assert_eq!(
            false,
            word_break(
                String::from("catsandog"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat")
                ]
            )
        );
    }
}
