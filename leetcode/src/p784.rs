// https://leetcode-cn.com/problems/letter-case-permutation/

pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut res = vec![];

    let l = s.len();
    backtrack(s, &mut res, String::new(), 0, l);
    res
}

fn backtrack(s: String, res: &mut Vec<String>, cur: String, start: usize, end: usize) {
    if start == end {
        res.push(cur);
        return;
    }
    let c: &char = &s.chars().nth(start).unwrap();
    let mut new_cur1 = cur.clone();
    new_cur1.push(*c);
    backtrack(s.clone(), res, new_cur1, start + 1, end);

    if c.is_ascii_alphabetic() {
        let mut new_cur2 = cur.clone();
        if c.is_ascii_uppercase() {
            new_cur2.push(c.to_ascii_lowercase());
        } else {
            new_cur2.push(c.to_ascii_uppercase());
        }
        backtrack(s.clone(), res, new_cur2, start + 1, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p784() {
        assert_eq!(
            vec!["a1b2", "a1B2", "A1b2", "A1B2"],
            letter_case_permutation(String::from("a1b2"))
        );
        assert_eq!(
            vec!["3z4", "3Z4"],
            letter_case_permutation(String::from("3z4"))
        );
        assert_eq!(
            vec!["12345"],
            letter_case_permutation(String::from("12345"))
        );
    }
}
