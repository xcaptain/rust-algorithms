//! This problem was asked by Microsoft.
//! Given a dictionary of words and a string made up of those words (no spaces), return the original sentence in a list.
//! If there is more than one possible reconstruction, return any of them. If there is no possible reconstruction, then return null.
//! For example, given the set of words 'quick', 'brown', 'the', 'fox', and the string "thequickbrownfox",
//! you should return ['the', 'quick', 'brown', 'fox'].
//! Given the set of words 'bed', 'bath', 'bedbath', 'and', 'beyond', and the string "bedbathandbeyond",
//! return either ['bed', 'bath', 'and', 'beyond] or ['bedbath', 'and', 'beyond'].
//!
//! Hint:
//! A very simple backtrack algorithms

pub fn construct_sentence(s: String, words: Vec<String>) -> Vec<Vec<String>> {
    let mut res = vec![];
    backtrack(&s, &words, 0, vec![], &mut res);
    res
}

fn backtrack(
    s: &String,
    words: &Vec<String>,
    start: usize,
    cur: Vec<String>,
    res: &mut Vec<Vec<String>>,
) {
    let end = s.len();
    if start == end {
        res.push(cur);
        return;
    }
    for i in start..end {
        let sub_s = &s[start..=i].to_owned();
        if words.contains(&sub_s) {
            let mut new_cur = cur.clone();
            new_cur.push(sub_s.clone());
            backtrack(s, words, i + 1, new_cur, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_sentence() {
        assert_eq!(
            vec![vec!["the", "quick", "brown", "fox"],],
            construct_sentence(
                String::from("thequickbrownfox"),
                vec![
                    String::from("quick"),
                    String::from("brown"),
                    String::from("the"),
                    String::from("fox"),
                ],
            )
        );

        assert_eq!(
            vec![
                vec!["bed", "bath", "and", "beyond"],
                vec!["bedbath", "and", "beyond"],
            ],
            construct_sentence(
                String::from("bedbathandbeyond"),
                vec![
                    String::from("bed"),
                    String::from("bath"),
                    String::from("bedbath"),
                    String::from("and"),
                    String::from("beyond"),
                ],
            )
        );
    }
}
