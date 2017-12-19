extern crate algorithms;

use algorithms::sort::merge_sort::merge_sort_v1;
use algorithms::sort::merge_sort::merge_sort_v2;

#[test]
fn test_normal() {
    assert_eq!(vec![1, 2, 3, 4], merge_sort_v1(vec![1, 2, 3, 4]));
    assert_eq!(vec![1, 2, 3, 4], merge_sort_v2(vec![1, 2, 3, 4]));
}

#[test]
fn test_reverse() {
    assert_eq!(vec![1, 2, 3, 4], merge_sort_v1(vec![4, 3, 2, 1]));
    assert_eq!(vec![1, 2, 3, 4], merge_sort_v2(vec![4, 3, 2, 1]));
}

#[test]
fn test_random() {
    assert_eq!(vec![1, 2, 3, 4], merge_sort_v1(vec![1, 3, 2, 4]));
    assert_eq!(vec![1, 2, 3, 4], merge_sort_v2(vec![1, 3, 2, 4]));
}
