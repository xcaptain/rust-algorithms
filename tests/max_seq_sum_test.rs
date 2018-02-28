extern crate algorithms;

use algorithms::iters::max_seq_sum::max_seq_sum_v1;
use algorithms::iters::max_seq_sum::max_seq_sum_v2;
use algorithms::iters::max_seq_sum::max_seq_sum_v3;

#[test]
fn test_default() {
    assert_eq!(15, max_seq_sum_v1(vec![1, 2, 3, 4, 5]));
    assert_eq!(15, max_seq_sum_v2(vec![1, 2, 3, 4, 5]));
    assert_eq!(15, max_seq_sum_v3(vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_all_neg() {
    assert_eq!(-1, max_seq_sum_v1(vec![-1, -2, -3, -4, -5]));
    assert_eq!(-1, max_seq_sum_v2(vec![-1, -2, -3, -4, -5]));
    assert_eq!(-1, max_seq_sum_v3(vec![-1, -2, -3, -4, -5]));
}

#[test]
fn test_some_neg() {
    assert_eq!(5, max_seq_sum_v1(vec![-1, -2, 3, -4, 5]));
    assert_eq!(5, max_seq_sum_v2(vec![-1, -2, 3, -4, 5]));
    assert_eq!(5, max_seq_sum_v3(vec![-1, -2, 3, -4, 5]));
}
