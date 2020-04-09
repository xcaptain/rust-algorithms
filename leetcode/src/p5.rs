// https://leetcode-cn.com/problems/longest-palindromic-substring/

pub fn longest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let mut start = 0_usize;
    let mut end = 0_usize;
    let l = s.len();
    for i in 0..l {
        // iterate over the string, the current element is the center of the substring
        let len1 = expand_around_center(&s, i, i);
        let len2 = expand_around_center(&s, i, i + 1);
        let len = len1.max(len2);
        if len > end - start {
            end = i + len / 2;
            start = i - (len - 1) / 2;
        }
    }
    String::from(&s[start..=end])
}

// use s.chars().nth(l) is slow here
fn expand_around_center(s: &str, left: usize, right: usize) -> usize {
    let mut l = left as i32;
    let mut r = right as i32;
    while l >= 0 && r < s.len() as i32 && s[l as usize..=l as usize] == s[r as usize..=r as usize] {
        l -= 1;
        r += 1;
    }
    (r - l - 1) as usize
}

/// expand from left, works but not very efficient, will out of time
pub fn longest_palindrome_v2(s: String) -> String {
    let mut res = String::new();
    let l = s.len();
    if l == 0 {
        return res;
    }

    for i in 0..l - 1 {
        for j in i..l {
            let sub_l = j - i + 1;
            if sub_l > res.len() && s[i..=i] == s[j..=j] && is_palindrome(&s[i..=j]) {
                res = s[i..=j].to_owned();
            }
        }
    }

    res
}

fn is_palindrome(s: &str) -> bool {
    let l = s.len();
    for i in 0..l / 2 {
        let j = l - i - 1;
        if s[i..=i] != s[j..=j] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5() {
        assert_eq!(
            String::from("aba"),
            longest_palindrome(String::from("babad"))
        );

        assert_eq!(
            String::from("bab"),
            longest_palindrome_v2(String::from("babad"))
        );
        assert_eq!(String::from(""), longest_palindrome_v2(String::from("")));
        assert_eq!(
            String::from("bb"),
            longest_palindrome_v2(String::from("cbbd"))
        );
    }
}
