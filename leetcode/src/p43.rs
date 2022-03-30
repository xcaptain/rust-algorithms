// https://leetcode-cn.com/problems/multiply-strings/

pub fn multiply(num1: String, num2: String) -> String {
    let mut res = String::from("0");
    if num1 == "0" || num2 == "0" {
        return res;
    }

    for i in (0..num2.len()).rev() {
        let mut temp = String::new();
        for _ in 0..num2.len() - i - 1 {
            temp.push('0');
        }
        let n2 = num2[i..=i].parse::<u32>().unwrap();
        let product = multiply_digit(num1.clone(), n2, 0);
        temp.insert_str(0, &product);
        res = add_strings(res, temp);
    }
    res
}

/// add 2 string numbers, copied from p415
fn add_strings(num1: String, num2: String) -> String {
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
        .trim_start_matches('0')
        .to_string()
}

/// multiply a string number with a digit, e.g. 888 * 9
/// zeros means add how many zeros at the end of product
fn multiply_digit(num1: String, c: u32, zeros: usize) -> String {
    let mut res = vec![];
    let num1: Vec<u32> = num1.chars().map(|e| e.to_digit(10).unwrap()).collect();
    let l1 = num1.len();
    let mut carry = 0;
    for i in 0..l1 {
        let n1 = num1[l1 - i - 1];
        let t = n1 * c + carry;
        res.insert(0, t % 10);
        carry = t / 10;
    }
    if carry > 0 {
        res.insert(0, carry);
    }

    if res[0] != 0 {
        res.resize(res.len() + zeros, 0);
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
    fn test_p43() {
        assert_eq!("60", multiply(String::from("2"), String::from("30")));
        assert_eq!("3741", multiply(String::from("87"), String::from("43")));
        assert_eq!("3741", multiply(String::from("43"), String::from("87")));
        assert_eq!("792", multiply_digit(String::from("88"), 9, 0));
        assert_eq!("0", multiply(String::from("9133"), String::from("0")));
    }
}
