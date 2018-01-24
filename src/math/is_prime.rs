// 判断一个正整数是否是素数
// 使用筛法判断是否能被小于它的数整除
pub fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
