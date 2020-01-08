// https://leetcode-cn.com/problems/number-of-segments-in-a-string/

pub fn count_segments(s: String) -> i32 {
    s.split(' ')
        .into_iter()
        .filter(|x: &&str| !x.trim().is_empty())
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p434() {
        assert_eq!(5, count_segments(String::from("Hello, my name is John")));
        assert_eq!(6, count_segments(String::from(", , , ,        a, eaefa")));
        assert_eq!(0, count_segments(String::from("")));
        assert_eq!(0, count_segments(String::from(" ")));
        assert_eq!(1, count_segments(String::from(" 1")));
    }
}
