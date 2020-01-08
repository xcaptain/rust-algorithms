// https://leetcode-cn.com/contest/weekly-contest-171/problems/minimum-flips-to-make-a-or-b-equal-to-c/

pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let a1 = format!("{:032b}", a);
    let b1 = format!("{:032b}", b);
    let c1 = format!("{:032b}", c);

    let mut ans = 0;
    for i in 0..32 {
        if &c1[i..=i] == "0"
            && ((&a1[i..=i] == "0" && &b1[i..=i] == "1")
                || (&a1[i..=i] == "1" && &b1[i..=i] == "0"))
        {
            ans += 1;
        } else if &c1[i..=i] == "0" && (&a1[i..=i] == "1" && &b1[i..=i] == "1") {
            ans += 2;
        } else if &c1[i..=i] == "1" && (&a1[i..=i] == "0" && &b1[i..=i] == "0") {
            ans += 1;
        }
    }
    ans
}

pub fn decbin(a: i32) -> String {
    format!("{:032b}", a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5308() {
        assert_eq!(3, min_flips(2, 6, 5));
        assert_eq!(1, min_flips(4, 2, 7));
        assert_eq!(0, min_flips(1, 2, 3));
    }

    #[test]
    fn test_decbin() {
        assert_eq!("00000000000000000000000000000010", decbin(2));
        assert_eq!("11111111111111111111111111110000", decbin(-16));
    }
}
