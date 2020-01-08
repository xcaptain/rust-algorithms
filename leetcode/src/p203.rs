// https://leetcode-cn.com/problems/remove-linked-list-elements/
// https://users.rust-lang.org/t/how-to-delete-element-in-linkedlist/34951/20
// TODO: don't know how to remove
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut cur_node = head.clone();
    while let Some(cur_node_val) = cur_node.as_mut() {
        // TODO: iterate over linked list
        // let mut next_node = cur_node_val.next;
        match cur_node_val.next {
            Some(ref next_node_val) => {
                if next_node_val.val == val {
                    cur_node_val.next = cur_node_val.next.unwrap().next;
                } else {
                    cur_node_val.next = cur_node_val.next;
                }
            },
            None => {}
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p203() {
        // 1->2->6->3->4->5->6, val = 6
        // 1->2->3->4->5
        let mut n1 = ListNode::new(1);
        let mut n2 = ListNode::new(2);
        let n6 = ListNode::new(6);
        let mut n3 = ListNode::new(3);
        let mut n4 = ListNode::new(4);
        let mut n5 = ListNode::new(5);
        let mut n6_1 = ListNode::new(6);

        n5.next = Some(Box::new(n6));
        n4.next = Some(Box::new(n5));
        n3.next = Some(Box::new(n4));
        n6_1.next = Some(Box::new(n3));
        n2.next = Some(Box::new(n6_1));
        n1.next = Some(Box::new(n2));

        let exp = remove_elements(Some(Box::new(n1)), 6);

        let mut e1 = ListNode::new(1);
        let mut e2 = ListNode::new(2);
        let mut e3 = ListNode::new(3);
        let mut e4 = ListNode::new(4);
        let e5 = ListNode::new(5);

        e4.next = Some(Box::new(e5));
        e3.next = Some(Box::new(e4));
        e2.next = Some(Box::new(e3));
        e1.next = Some(Box::new(e2));

        assert_eq!(exp, Some(Box::new(e1)));
    }
}
