extern crate algorithms;

use algorithms::sort::selection_sort::selection_sort;

#[test]
fn test_normal() {
    assert_eq!(vec![1, 2, 3, 4], selection_sort(vec![1, 2, 3, 4]));
}

#[test]
fn test_random() {
    assert_eq!(vec![1, 2, 3, 4], selection_sort(vec![1, 3, 2, 4]));
}
