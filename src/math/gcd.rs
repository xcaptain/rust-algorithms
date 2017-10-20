// 最大公约数算法，m,n >= 0
// TODO: 如果m,n差距很大的话会有性能问题，需要找个更加优化的算法
pub fn gcd(m: usize, n: usize) -> usize {
    let g = m % n;
    if g == 0 {
        return n;
    }
    return gcd(n, g);
}
