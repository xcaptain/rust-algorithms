// https://leetcode-cn.com/problems/reverse-bits/

pub fn reverse_bits(n: u32) -> u32 {
    let mut num = n;
    let mut arr = vec![0; 32];
    let mut i = 0;
    while num > 0 {
        let rem = num % 2;
        arr[i] = rem;
        num /= 2;
        i += 1;
    }
    // println!("arr={:?}", arr);
    let mut res = 0;
    for (k, bit) in arr.iter().enumerate() {
        let t = *bit * 2u32.pow((31 - k) as u32);
        res += t;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(964176192, reverse_bits(43261596));
        assert_eq!(3221225471, reverse_bits(4294967293));
    }
}
