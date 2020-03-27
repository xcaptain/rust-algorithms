//! This problem was asked by Amazon.
//! Given an integer k and a string s, find the length of the longest substring that contains at most k distinct characters.
//! For example, given s = "abcba" and k = 2, the longest substring with k distinct characters is "bcb".

use std::collections::HashMap;

pub fn get_at_most_k_distinct(s: String, k: usize) -> String {
    let mut longest_s = String::new();
    let l = s.len();

    for i in 0..l - 1 {
        for j in i + 1..l {
            let mut exp_l = j - i + 1;
            if exp_l < longest_s.len() {
                continue;
            }
            let distinct_num = get_distinct_chars(&s[i..=j]);
            if distinct_num == k {
                // look forward, to check duplicate
                let mut end_num = 0;
                for k in j + 1..l {
                    if s[i..=j].contains(&s[k..=k]) {
                        end_num += 1;
                    } else {
                        break;
                    }
                }
                exp_l += end_num;
                // println!("{}, {}, {}", i, j, end_num);
                if exp_l > longest_s.len() {
                    longest_s = s[i..=(j + end_num)].to_owned();
                }
                break;
            }
        }
    }
    longest_s
}

fn get_distinct_chars(s: &str) -> usize {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }

    map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_at_most_k_distinct() {
        assert_eq!(
            String::from("bcb"),
            get_at_most_k_distinct(String::from("abcba"), 2)
        );

        assert_eq!(
            String::from("cbcbc"),
            get_at_most_k_distinct(String::from("acbcbca"), 2)
        );
    }

    #[test]
    fn test_get_distinct_chars() {
        assert_eq!(3, get_distinct_chars("abc"));
        assert_eq!(3, get_distinct_chars("abcc"));
    }
}
