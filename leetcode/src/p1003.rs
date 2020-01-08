// https://leetcode-cn.com/problems/check-if-word-is-valid-after-substitutions/

pub fn is_valid(s: String) -> bool {
    let mut s = s;
    while s.len() >= 3 {
        let ns = s.replace("abc", "");
        if ns == s {
            return false;
        }
        s = ns;
        if s == "" || s == "abc" {
            return true;
        }
    }
    false
}

pub fn is_valid_v2(s: String) -> bool {
    let mut st: Vec<char> = vec![];
    for c in s.chars() {
        if c != 'c' {
            st.push(c);
        } else {
            if st.len() < 2 {
                return false;
            }
            // println!("st: {:?}, last: {}, prev: {}, pprev: {}", st, st.last().unwrap(), st[0], st[1]);
            // how to get last and last last?
            if st[st.len() - 1] != 'b' || st[st.len() - 2] != 'a' {
                return false;
            }
            st.pop();
            st.pop();
        }
    }
    st.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1003() {
        assert_eq!(true, is_valid(String::from("aabcbc")));
        assert_eq!(true, is_valid(String::from("abcabcababcc")));
        assert_eq!(false, is_valid(String::from("abccba")));
        assert_eq!(false, is_valid(String::from("cababc")));

        assert_eq!(true, is_valid_v2(String::from("aabcbc")));
        assert_eq!(true, is_valid_v2(String::from("abcabcababcc")));
        assert_eq!(false, is_valid_v2(String::from("abccba")));
        assert_eq!(false, is_valid_v2(String::from("cababc")));
    }
}
