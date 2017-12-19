extern crate algorithms;

use algorithms::linkedlist::double_linked_list::List;
use algorithms::linkedlist::double_linked_list::Link;

#[test]
fn test_new() {
    let l = List::new();
    assert_eq!(l.head, Link::Empty);
}