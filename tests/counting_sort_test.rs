extern crate algorithms;

use algorithms::sort::counting_sort::counting_sort;

#[test]
fn test_normal() {
    assert_eq!(vec![1, 2, 3, 4], counting_sort(vec![1, 2, 3, 4], 100));
}

#[test]
fn test_reverse() {
    assert_eq!(vec![1, 1, 2, 3, 4], counting_sort(vec![4, 1, 3, 2, 1], 100));
}

#[test]
fn test_random() {
    assert_eq!(vec![1, 2, 3, 4], counting_sort(vec![1, 3, 2, 4], 100));
}
