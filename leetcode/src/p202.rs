// https://leetcode-cn.com/problems/happy-number/

pub fn is_happy(num: i32) -> bool {
    let mut res = num;
    let mut computed = vec![res];
    while res > 1 {
        let mut p = res;
        let mut s = 0;
        while p > 0 {
            let q = p % 10;
            s += q * q;
            p /= 10;
        }
        res = s;
        if computed.contains(&res) {
            return false;
        }
        computed.push(res);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p202() {
        assert!(is_happy(19));
        assert!(is_happy(7));
        assert_eq!(false, is_happy(2));
    }
}
