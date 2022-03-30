// https://leetcode-cn.com/problems/top-k-frequent-words/

use std::cmp::Ordering;
use std::collections::HashMap;

pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut map: HashMap<String, i32> = HashMap::new();
    for word in words {
        let counter = map.entry(word.clone()).or_insert(0);
        *counter += 1;
    }

    let mut arr: Vec<(String, i32)> = vec![];
    for (key, value) in map {
        arr.push((key, value));
    }
    arr.sort_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.0.cmp(&b.0),
    });

    let mut res = vec![];
    for i in 0..k as usize {
        res.push(arr[i].0.clone());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p692() {
        assert_eq!(
            vec![String::from("i"), String::from("love")],
            top_k_frequent(
                vec![
                    String::from("i"),
                    String::from("love"),
                    String::from("leetcode"),
                    String::from("i"),
                    String::from("love"),
                    String::from("coding")
                ],
                2
            )
        );

        assert_eq!(
            vec![
                String::from("the"),
                String::from("is"),
                String::from("sunny"),
                String::from("day")
            ],
            top_k_frequent(
                vec![
                    String::from("the"),
                    String::from("day"),
                    String::from("is"),
                    String::from("sunny"),
                    String::from("the"),
                    String::from("the"),
                    String::from("the"),
                    String::from("sunny"),
                    String::from("is"),
                    String::from("is")
                ],
                4
            )
        );
    }
}
