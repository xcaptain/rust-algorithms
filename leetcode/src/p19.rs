// https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// loop twice, return the new list
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut l = 0; // 链表长度
    let mut head = head;
    let mut cur_node = &mut head;
    while let Some(node) = cur_node.as_mut() {
        l += 1;
        cur_node = &mut node.next;
    }
    if l == 1 {
        return None;
    }
    // 删除第 l - n 个元素，从0开始数
    let mut cur_node = &mut head;
    let mut i = 1;
    while let Some(node) = cur_node.as_mut() {
        if i == l - n {
            if node.next.is_none() {
                return None;
            }
            node.next.take().map(|next_node| {
                node.next = next_node.next;
            });
        }
        cur_node = &mut node.next;
        i += 1;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p19() {
        let mut n1 = Box::new(ListNode::new(1));
        let mut n2 = Box::new(ListNode::new(2));
        let mut n3 = Box::new(ListNode::new(3));
        let mut n4 = Box::new(ListNode::new(4));
        let n5 = Box::new(ListNode::new(5));

        n4.next = Some(n5);
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);

        let nl = remove_nth_from_end(Some(n1), 2);
        assert_eq!(5, nl.unwrap().next.unwrap().next.unwrap().next.unwrap().val);
    }
}
