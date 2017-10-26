extern crate algorithms;

use algorithms::linkedlist::simple_list::SimpleList as List;

#[test]
fn test_empty_list() {
    let l = List::new();
    assert_eq!(0, l.len());
}

#[test]
fn test_prepend() {
    let mut l = List::new();
    l = l.prepend(10);
    assert_eq!(1, l.len());
}

#[test]
fn test_stringify() {
    let mut l = List::new();
    l = l.prepend(10);
    assert_eq!(String::from("10, Nil"), l.stringify());
}
