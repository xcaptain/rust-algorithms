// https://leetcode-cn.com/problems/add-strings/

pub fn add_strings(num1: String, num2: String) -> String {
    let num1: Vec<char> = num1.chars().collect();
    let num2: Vec<char> = num2.chars().collect();
    let l1 = num1.len();
    let l2 = num2.len();
    let l = l1.max(l2);
    let mut carry = 0;
    let mut res: Vec<u32> = vec![];
    for i in 0..l {
        let n1 = if l1 > i {
            num1[l1 - i - 1].to_digit(10).unwrap()
        } else {
            0
        };
        let n2 = if l2 > i {
            num2[l2 - i - 1].to_digit(10).unwrap()
        } else {
            0
        };
        let t = n1 + n2 + carry;
        res.insert(0, t % 10);
        carry = t / 10;
    }
    if carry > 0 {
        res.insert(0, carry);
    }

    res.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p415() {
        assert_eq!("41", add_strings(String::from("12"), String::from("29")));
        assert_eq!("21", add_strings(String::from("12"), String::from("9")));
        assert_eq!("187", add_strings(String::from("88"), String::from("99")));
        assert_eq!("88", add_strings(String::from("88"), String::from("0")));
        assert_eq!(
            "1111111110",
            add_strings(String::from("123456789"), String::from("987654321"))
        );
    }
}
