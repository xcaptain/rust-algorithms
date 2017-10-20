extern crate algorithms;

use algorithms::math::is_prime::is_prime;

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


#[test]
// fn test_very_large_prime() { // 暂时不测试大素数
//     assert_eq!(true, is_prime(170141183460469231731687303715884105727));
// }
