// https://leetcode-cn.com/problems/implement-strstr/

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_len = haystack.len();
    let needle_len = needle.len();
    if haystack_len < needle_len {
        // needle is longger return -1 (NOT FOUND)
        return -1;
    }
    // move from a window and compare the 2 substring is equal
    let mut res: i32 = -1;
    for i in 0..=haystack_len - needle_len {
        let mut window_match = true;
        for j in 0..needle_len {
            let current_needle_char = needle.chars().nth(j);
            let current_haystack_char = haystack.chars().nth(i + j);
            if current_needle_char != current_haystack_char {
                window_match = false;
            }
        }
        if window_match == true {
            // the whole window matched, stop and return res
            res = i as i32;
            break;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(2, str_str(String::from("hello"), String::from("ll")));
        assert_eq!(0, str_str(String::from("hello"), String::from("")));
        assert_eq!(-1, str_str(String::from("hello"), String::from("lh")));
    }
}
