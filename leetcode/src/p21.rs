// https://leetcode-cn.com/problems/merge-two-sorted-lists/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut ll1 = l1.as_ref();
    let mut ll2 = l2.as_ref();

    let mut l1_vals: Vec<i32> = vec![];
    let mut l2_vals: Vec<i32> = vec![];
    while let Some(l1_node) = ll1 {
        l1_vals.push(l1_node.val);
        ll1 = l1_node.next.as_ref();
    }
    while let Some(l2_node) = ll2 {
        l2_vals.push(l2_node.val);
        ll2 = l2_node.next.as_ref();
    }
    let mut l3_vals = merge(l1_vals, l2_vals);
    l3_vals.reverse();

    let mut l3: Option<Box<ListNode>> = None;
    for item3 in l3_vals {
        l3 = Some(Box::new(ListNode {
            val: item3,
            next: l3.take(),
        }));
    }
    return l3;
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut left = left;
    let mut right = right;
    let mut res = vec![];
    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            res.push(left[0]);
            left.remove(0);
        } else {
            res.push(right[0]);
            right.remove(0);
        }
    }
    if !left.is_empty() {
        res.extend(&left);
    }
    if !right.is_empty() {
        res.extend(&right);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let l3 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(l3, merge_two_lists(l1, l2));
    }
}
