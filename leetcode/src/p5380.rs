// https://leetcode-cn.com/problems/string-matching-in-an-array/

pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let l = words.len();

    let mut res = vec![];
    let mut is_substring = vec![false; l];
    for i in 0..l {
        for j in 0..l {
            if words[i].len() > words[j].len() && words[i].contains(&words[j]) && !is_substring[j] {
                res.push(words[j].clone());
                is_substring[j] = true;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5380() {
        assert_eq!(
            vec!["as", "hero"],
            string_matching(vec![
                String::from("mass"),
                String::from("as"),
                String::from("hero"),
                String::from("superhero"),
            ])
        );
        assert_eq!(
            vec!["leetcode", "od", "am"],
            string_matching(vec![
                String::from("leetcoder"),
                String::from("leetcode"),
                String::from("od"),
                String::from("hamlet"),
                String::from("am")
            ])
        );
    }
}
