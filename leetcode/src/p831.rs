pub fn mask_pii(s: String) -> String {
    if s.contains('@') {
        return mask_email(s);
    }
    mask_phone(s)
}

fn mask_email(s: String) -> String {
    let s = s.to_ascii_lowercase(); // 小写
    let names: Vec<String> = s.split('@').map(|e| String::from(e)).collect();
    let name1 = names[0].clone();
    let new_name1 = format!(
        "{}*****{}",
        name1.chars().nth(0).unwrap(),
        name1.chars().nth(name1.len() - 1).unwrap()
    );
    format!("{}@{}", new_name1, names[1])
}

fn mask_phone(s: String) -> String {
    let mut numbers: Vec<u32> = vec![];
    for c in s.chars() {
        if c.is_digit(10) {
            numbers.push(c.to_digit(10).unwrap());
        }
    }
    if numbers.len() == 10 {
        return format!(
            "***-***-{}{}{}{}",
            numbers[6], numbers[7], numbers[8], numbers[9]
        );
    } else if numbers.len() == 11 {
        return format!(
            "+*-***-***-{}{}{}{}",
            numbers[7], numbers[8], numbers[9], numbers[10]
        );
    } else if numbers.len() == 12 {
        return format!(
            "+**-***-***-{}{}{}{}",
            numbers[8], numbers[9], numbers[10], numbers[11]
        );
    } else if numbers.len() == 13 {
        return format!(
            "+***-***-***-{}{}{}{}",
            numbers[9], numbers[10], numbers[11], numbers[12]
        );
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p831() {
        assert_eq!(
            "l*****e@leetcode.com",
            mask_pii(String::from("LeetCode@LeetCode.com"))
        );
        assert_eq!("a*****b@qq.com", mask_pii(String::from("AB@qq.com")));
        assert_eq!("***-***-7890", mask_pii(String::from("1(234)567-890")));
        assert_eq!(
            "+**-***-***-5678",
            mask_pii(String::from("86-(10)12345678"))
        );
        assert_eq!(
            "+***-***-***-3431",
            mask_pii(String::from("+(501321)-50-23431"))
        );
        assert_eq!(
            "+*-***-***-3774",
            mask_pii(String::from("+86(88)1513-7-74"))
        );
    }
}
