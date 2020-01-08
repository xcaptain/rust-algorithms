// https://leetcode-cn.com/problems/reverse-words-in-a-string-iii/

pub fn reverse_words(s: String) -> String {
    s.split(' ')
        .map(|e| {
            let l = e.len();
            if l == 0 {
                // 判断边界
                return String::from("");
            }
            let mut left = 0;
            let mut right = l - 1;
            let mut ee: Vec<char> = e.chars().collect();
            while left < right {
                ee.swap(left, right);
                left += 1;
                right -= 1;
            }
            ee.iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p557() {
        assert_eq!(
            "s'teL ekat edoCteeL tsetnoc",
            reverse_words(String::from("Let's take LeetCode contest"))
        );
        assert_eq!("", reverse_words(String::from("")));
    }
}
