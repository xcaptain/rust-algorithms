extern crate algorithms;

use algorithms::iters::two_sum::two_sum;

#[test]
fn test_normal() {
    assert_eq!((0, 2), two_sum(vec![1,2,3], 4));
}

#[test]
fn test_duplicate_elem() {
    assert_eq!((0, 2), two_sum(vec![2, 1, 2, 3], 4));
}

#[test]
fn test_more_duplicate_elem() {
    // 这里返回的不是0, 1是因为rust的数组按值查找用的是binary_search
    // 所以找到的index不一定是第一次出现的位置
    assert_eq!((0, 2), two_sum(vec![2, 2, 2, 3], 4));
}
