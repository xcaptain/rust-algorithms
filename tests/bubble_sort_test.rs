extern crate algorithms;

use algorithms::sort::bubble_sort;

#[test]
fn test_normal() {
    assert_eq!(String::from("hello"), bubble_sort());
}

#[test]
fn test_fail() {
    assert_eq!(String::from("hello world"), bubble_sort());
}
