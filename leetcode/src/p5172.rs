// https://leetcode-cn.com/problems/largest-multiple-of-three/

pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
    let s = digits.iter().sum::<i32>();
    let r = s % 3;
    let mut digits = digits;
    digits.sort_by(|a, b| b.cmp(&a)); // 从大到小排列
    if digits[0] == 0 {
        return String::from("0");
    }
    let l = digits.len();
    let mut bo: Vec<bool> = vec![true; l];
    if r == 0 {
        // 直接返回
    } else if r == 1 {
        // 查找digits是否包含1，如果有则删除之
        if let Some(pos) = digits.iter().position(|&d| d % 3 == 1) {
            // println!("%1, {}", l-pos-1);
            bo[l - pos - 1] = false;
        } else {
            // 删除最小的2个余2的
            let mut cnt = 0;
            for i in (0..l).rev() {
                if cnt >= 2 {
                    break;
                }
                if digits[i] % 3 == 2 {
                    cnt += 1;
                    bo[i] = false;
                }
            }
        }
    } else {
        if let Some(pos) = digits.iter().rev().position(|&d| d % 3 == 2) {
            bo[l - pos - 1] = false;
        } else {
            // 删除最小的2个余1的
            let mut cnt = 0;
            for i in (0..l).rev() {
                if cnt >= 2 {
                    break;
                }
                if digits[i] % 3 == 1 {
                    cnt += 1;
                    bo[i] = false;
                }
            }
        }
    }
    let mut ans = String::new();
    for i in 0..l {
        let item = digits[i];
        if bo[i] {
            ans.push_str(&item.to_string());
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5172() {
        assert_eq!("981", largest_multiple_of_three(vec![8, 1, 9]));
        assert_eq!("9831", largest_multiple_of_three(vec![8, 1, 9, 3]));
        assert_eq!("8760", largest_multiple_of_three(vec![8, 6, 7, 1, 0]));
        assert_eq!("", largest_multiple_of_three(vec![1]));
        assert_eq!("0", largest_multiple_of_three(vec![0, 0, 0, 0, 0, 0]));
    }
}
