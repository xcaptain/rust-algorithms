// from scipy.special import comb, perm

pub fn perm1(n: usize, k: usize) -> usize {
    let mut r = 1;
    let mut nn = n;
    let mut i = 0;
    while i < k {
        r *= nn;
        nn -= 1;
        i += 1;
    }
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perm() {
        assert_eq!(12, perm1(4, 2));
        assert_eq!(20, perm1(5, 2));
        assert_eq!(5079110400, perm1(20, 8));
    }
}
