// https://leetcode-cn.com/problems/simplify-path/

pub fn simplify_path(path: String) -> String {
    let arr: Vec<String> = path.split('/').map(String::from).collect();
    let mut st: Vec<String> = vec![];
    for s in arr {
        if !st.is_empty() && s == ".." {
            st.pop();
        } else if !s.is_empty() && s != "." && s != ".." {
            st.push(s)
        }
    }
    if st.is_empty() {
        return String::from("/");
    }
    let mut res = String::new();
    for item in st {
        res = format!("{}/{}", res, item);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p71() {
        assert_eq!("/home", simplify_path(String::from("/home/")));
    }
}
