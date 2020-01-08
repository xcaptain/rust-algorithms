// https://leetcode-cn.com/problems/sum-of-square-numbers/

pub fn judge_square_sum(c: i32) -> bool {
    let mid = (c as f64).sqrt() as i32;
    for a in 0..=mid {
        let exp_b = ((c - a * a) as f64).sqrt() as i32;
        if exp_b * exp_b + a * a == c {
            // println!("{}, {}, {}", a, exp_b, c);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p633() {
        assert_eq!(true, judge_square_sum(8));
        assert_eq!(true, judge_square_sum(5));
        assert_eq!(true, judge_square_sum(4));
        assert_eq!(false, judge_square_sum(3));
        assert_eq!(true, judge_square_sum(1));
    }
}
