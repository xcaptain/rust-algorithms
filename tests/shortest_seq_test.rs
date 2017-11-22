extern crate algorithms;

use algorithms::iters::shortest_seq::shortest_seq;

#[test]
fn test_1() {
    assert_eq!((3, 2, 4), shortest_seq(vec![1,2,3,5,6], 12));
}

#[test]
fn test_2() {
    assert_eq!((2, 3, 4), shortest_seq(vec![1,2,3,5,6], 11));
}

#[test]
fn test_3() {
    assert_eq!((2, 3, 4), shortest_seq(vec![1,2,3,5,6], 10));
}

#[test]
fn test_4() {
    assert_eq!((0, 0, 0), shortest_seq(vec![1,2,3,5,6], 20));
}
