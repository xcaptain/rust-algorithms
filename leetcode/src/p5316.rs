// https://leetcode-cn.com/contest/weekly-contest-172/problems/print-words-vertically/

pub fn print_vertically(s: String) -> Vec<String> {
    let mut ans: Vec<String> = vec![String::from("")];
    let words: Vec<String> = s.split(' ').map(String::from).collect();
    for word in words {
        let ans_len = ans.len();
        for (i, c) in word.chars().enumerate() {
            if i < ans_len {
                ans[i] += &c.to_string();
            } else {
                // left pad space
                let mut last_word = String::from("");
                let space_len = ans[0].len();
                if space_len > 1 {
                    for _j in 0..space_len - 1 {
                        last_word.push(' ');
                    }
                }

                last_word.push(c);
                ans.push(last_word);
            }
        }
        let ans0_len = ans[0].len();
        for i in 0..ans_len {
            if ans[i].len() < ans0_len {
                ans[i].push(' ');
            }
        }
    }
    for i in 0..ans.len() {
        ans[i] = ans[i].trim_end().to_string();
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5316() {
        assert_eq!(
            vec!["HAY", "ORO", "WEU"],
            print_vertically(String::from("HOW ARE YOU"))
        );
        assert_eq!(
            vec!["TBONTB", "OEROOE", "   T"],
            print_vertically(String::from("TO BE OR NOT TO BE"))
        );
        assert_eq!(
            vec!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"],
            print_vertically(String::from("CONTEST IS COMING"))
        );
    }
}
