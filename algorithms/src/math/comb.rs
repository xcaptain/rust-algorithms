// from scipy.special import comb, perm

pub fn comb1(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let mut r = 1;
    let mut nn = n;
    // using this trick can avoid larger middle result overflow
    for d in 1..=k {
        r *= nn;
        nn -= 1;
        r /= d;
    }
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        assert_eq!(6, comb1(4, 2));
        assert_eq!(10, comb1(5, 2));
        assert_eq!(5567902560, comb1(36, 15));
    }
}
