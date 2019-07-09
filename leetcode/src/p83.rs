// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/

use std::collections::HashMap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// because box can only have 1 owner, so can't modify inplace, must return a new list
// use a hashmap to reduce duplicates
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head.as_ref();
    let mut m: HashMap<i32, bool> = HashMap::new(); // check unique
    let mut uniq_arr = vec![];
    while let Some(cur_node) = cur {
        let cached_val = m.get(&cur_node.val);
        if cached_val.is_none() {
            m.insert(cur_node.val, true);
            uniq_arr.push(cur_node.val);
        }
        cur = cur_node.next.as_ref();
    }

    // create a new list from uniq_arr
    uniq_arr.reverse();
    let mut l3: Option<Box<ListNode>> = None;
    for item3 in uniq_arr {
        l3 = Some(Box::new(ListNode {
            val: item3,
            next: l3.take(),
        }));
    }
    return l3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        assert_eq!(l2, delete_duplicates(l1));
    }
}
