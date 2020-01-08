// https://leetcode-cn.com/problems/reverse-string/

pub fn reverse_string(s: &mut Vec<char>) {
    let l = s.len();
    if l == 0 {
        return;
    }
    let mut left = 0;
    let mut right = l - 1;
    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p344() {
        let mut s1 = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s1);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], s1);

        let mut s2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s2);
        assert_eq!(vec!['h', 'a', 'n', 'n', 'a', 'H'], s2);
    }
}
