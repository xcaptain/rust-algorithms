extern crate algorithms;

use algorithms::sort::bubble_sort::bubble_sort;

#[test]
fn test_normal() {
    assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![1, 2, 3, 4]));
}

#[test]
fn test_reverse() {
    assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![4, 3, 2, 1]));
}

#[test]
fn test_random() {
    assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![1, 3, 2, 4]));
}
