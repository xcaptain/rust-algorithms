pub fn hamming_weight(n: u32) -> i32 {
    let mut mask = 1;
    let mut ans = 0;
    for _i in 0..32 {
        if n & mask != 0 {
            ans += 1;
        }
        mask = mask << 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p191() {
        assert_eq!(3, hamming_weight(11));
        assert_eq!(1, hamming_weight(128));
    }
}
