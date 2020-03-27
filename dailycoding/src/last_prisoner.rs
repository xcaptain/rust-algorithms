pub fn get_last_prisoner(n: usize, k: usize) -> usize {
    let mut flag = vec![1; n];

    let mut out_cnt = 0;
    let mut num_off = 0;
    while out_cnt < n - 1 {
        for i in 0..n {
            if flag[i] == 1 {
                num_off += 1;
                // 如果达到了最后一个位置，这时num_off还不到k，通过while
                // 循环再从头for循环一遍
                if num_off == k {
                    out_cnt += 1;
                    flag[i] = 0;
                    num_off = 0;
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..n {
        if flag[i] == 1 {
            res = i + 1;
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_last_prisoner() {
        assert_eq!(3, get_last_prisoner(5, 2));
    }
}
