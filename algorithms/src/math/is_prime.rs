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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2() {
        assert_eq!(true, is_prime(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, is_prime(3));
    }

    #[test]
    fn test_4() {
        assert_eq!(false, is_prime(4));
    }

    // #[test]
    // fn test_very_large_prime() { // 暂时不测试大素数
    //     assert_eq!(true, is_prime(170141183460469231731687303715884105727));
    // }
}
