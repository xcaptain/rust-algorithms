extern crate algorithms;

use algorithms::sort::insertion_sort::insertion_sort;

#[test]
fn test_normal() {
    assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![1, 2, 3, 4]));
}

#[test]
fn test_reverse() {
    assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![4, 3, 2, 1]));
}

#[test]
fn test_random() {
    assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![1, 3, 2, 4]));
}
