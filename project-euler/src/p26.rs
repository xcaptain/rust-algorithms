/// 所有小于n的正整数中，最大的循环位数
pub fn longest_cycle(n: usize) -> usize {
    let mut longest_cyc_num = 0;
    let mut lognest_cyc = 0;
    for i in 1..n {
        let rc = get_rec_cycle(i);
        if rc >= lognest_cyc {
            lognest_cyc = rc;
            longest_cyc_num = i;
        }
    }
    longest_cyc_num
}

/// 获取数字q对应的循环位数, p/q
pub fn get_rec_cycle(q: usize) -> usize {
    let mut p = 1;
    let mut pa: Vec<usize> = vec![];
    loop {
        let m = p % q;
        if m == 0 {
            return 0;
        }
        p = m * 10;
        if pa.contains(&m) {
            let index = pa.iter().position(|&r| r == m).unwrap();
            return pa.len() - index;
        }
        pa.push(m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_cycle() {
        assert_eq!(7, longest_cycle(10));
        assert_eq!(983, longest_cycle(1000));
    }

    #[test]
    fn test_get_rec_cycle() {
        assert_eq!(6, get_rec_cycle(7));
        assert_eq!(1, get_rec_cycle(3));
        assert_eq!(1, get_rec_cycle(6));
        assert_eq!(1, get_rec_cycle(9));
        assert_eq!(0, get_rec_cycle(5));
        assert_eq!(2, get_rec_cycle(11));
        assert_eq!(0, get_rec_cycle(10));
        assert_eq!(982, get_rec_cycle(983));
    }
}
