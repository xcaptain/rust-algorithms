// 最小公倍数算法

use math::gcd::gcd;

// 利用最大公约数计算
pub fn lcm(m: usize, n: usize) -> usize {
    let g = gcd(m, n);
    return m*n/g;
}
