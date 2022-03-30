// https://leetcode-cn.com/problems/greatest-common-divisor-of-strings/

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let shorter: String = if str1.len() >= str2.len() {
        str2.clone()
    } else {
        str1.clone()
    };

    // 必须要两次循环才能把更短字符串全部切分出来
    let l = shorter.len();
    let mut res = String::from("");
    for i in 0..l - 1 {
        for j in i + 1..l {
            let sls = &shorter[i..=j];
            if is_divisor(&str1, sls) && is_divisor(&str2, sls) && (j - i) > res.len() {
                res = sls.to_owned();
            }
        }
    }
    res
}

fn is_divisor(s1: &str, divisor: &str) -> bool {
    let l1 = s1.len();
    let l2 = divisor.len();
    if l1 % l2 != 0 {
        return false;
    }
    let s2 = divisor.repeat(l1 / l2);
    s2 == s1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1071() {
        assert_eq!(
            "ABC",
            gcd_of_strings(String::from("ABCABC"), String::from("ABC"))
        );
        assert_eq!(
            "AB",
            gcd_of_strings(String::from("ABAB"), String::from("ABABAB"))
        );
        assert_eq!(
            "AAAAAAAAAA",
            gcd_of_strings(String::from("AAAAAAAAAA"), String::from("AAAAAAAAAA"))
        );
        assert_eq!(
            "",
            gcd_of_strings(String::from("LEET"), String::from("CODE"))
        );

        assert!(is_divisor("ABAB", "AB"));
        assert_eq!(false, is_divisor("LEET", "CODE"));
        assert!(is_divisor("AAAAAAAAAA", "AAAAAAAAAA"));
    }
}
