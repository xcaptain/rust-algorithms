// https://leetcode-cn.com/problems/reverse-string-ii/

pub fn reverse_str(s: String, k: i32) -> String {
    let mut start = 0;
    let step = 2 * k;
    let l = s.len() as i32;
    let mut ss: Vec<char> = s.chars().collect();
    while start < l {
        // rotate first k chars
        if start + step < l {
            // 翻转2k中的前k个
            let mut left = start;
            let mut right = start + k - 1;
            while left < right {
                ss.swap(left as usize, right as usize);
                left += 1;
                right -= 1;
            }
        } else {
            let rem = l - start;
            let mut right = if rem < k { l - 1 } else { start + k - 1 };
            let mut left = start;
            while left < right {
                ss.swap(left as usize, right as usize);
                left += 1;
                right -= 1;
            }
        }

        start += step;
    }
    ss.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p541() {
        assert_eq!("bacdfeg", reverse_str(String::from("abcdefg"), 2));
        assert_eq!("a", reverse_str(String::from("a"), 2));
    }
}
