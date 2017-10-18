extern crate algorithms;

use algorithms::search::binary_search::binary_search_rec;
use algorithms::search::binary_search::binary_search_iter;

#[test]
fn test_normal_rec() {
    assert_eq!(Some(0), binary_search_rec(vec![1,2,3], 1));
}

#[test]
fn test_not_found_rec() {
    assert_eq!(None, binary_search_rec(vec![1,2,3], 10));
}

#[test]
fn test_greater_rec() {
    assert_eq!(Some(2), binary_search_rec(vec![1,2,3], 3));
}


// for iter version
#[test]
fn test_normal_iter() {
    assert_eq!(Some(0), binary_search_iter(vec![1,2,3], 1));
}

#[test]
fn test_not_found_iter() {
    assert_eq!(None, binary_search_iter(vec![1,2,3], 10));
}

#[test]
fn test_greater_iter() {
    assert_eq!(Some(2), binary_search_iter(vec![1,2,3], 3));
}
