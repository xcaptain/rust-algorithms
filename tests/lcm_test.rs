extern crate algorithms;

use algorithms::math::lcm::lcm;

#[test]
fn test_1() {
    assert_eq!(2, lcm(1, 2));
}

#[test]
fn test_zero() {
    assert_eq!(0, lcm(0, 10));
}
