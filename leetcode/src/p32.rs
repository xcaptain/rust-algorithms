// https://leetcode-cn.com/problems/longest-valid-parentheses/

// ())
// st: -1
// st: 0, -1
// st: -1, maxans: 2
// st: 2

pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut res = 0;
    let mut st: Vec<i32> = vec![-1]; // stack hold index of parenthe
    for i in 0..s.len() {
        if &s[i..=i] == "(" {
            st.push(i as i32); // keeps the earliest start of parenthe
        } else {
            st.pop();
            if st.is_empty() {
                // not balance, update position to i
                st.push(i as i32);
            } else {
                res = res.max((i as i32) - st.last().unwrap());
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p32() {
        assert_eq!(2, longest_valid_parentheses(String::from("(()")));
        assert_eq!(4, longest_valid_parentheses(String::from(")()())")));
    }
}
