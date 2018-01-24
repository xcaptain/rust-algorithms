extern crate algorithms;

use algorithms::iters::add_two_sum::add_two_sum;

#[test]
fn test_default() {
    assert_eq!(vec![7, 0, 8], add_two_sum(vec![2, 4, 3], vec![5, 6, 4]));
}
