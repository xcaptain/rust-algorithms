pub fn is_valid(s: String) -> bool {
    let vs: Vec<char> = s.chars().collect();
    let mut st: Vec<char> = vec![];
    for c in &vs {
        if *c == '(' || *c == '[' || *c == '{' {
            st.push(*c);
        } else if *c == ')' || *c == ']' || *c == '}' {
            let l = st.len();
            if l == 0 {
                return false;
            }
            let last_char = st[l - 1];
            if last_char == '(' && *c == ')' {
                st.pop();
            } else if last_char == '[' && *c == ']' {
                st.pop();
            } else if last_char == '{' && *c == '}' {
                st.pop();
            } else {
                st.push(*c);
            }
        }
    }
    if st.is_empty() {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(true, is_valid("()".to_string()));
        assert_eq!(true, is_valid("()[]{}".to_string()));
        assert_eq!(false, is_valid("(]".to_string()));
        assert_eq!(false, is_valid("([)]".to_string()));
        assert_eq!(true, is_valid("{[]}".to_string()));
        assert_eq!(false, is_valid("([]".to_string()));
        assert_eq!(false, is_valid("(])".to_string()));
    }
}
