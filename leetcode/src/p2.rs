// https://leetcode-cn.com/problems/add-two-numbers/

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

pub fn add_two_numbers(
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

    let mut l1_num = 0;
    for (k, v) in l1_vals.iter().enumerate() {
        l1_num += (*v as usize) * 10usize.pow(k as u32);
    }

    let mut l2_num = 0;
    for (k, v) in l2_vals.iter().enumerate() {
        l2_num += (*v as usize) * 10usize.pow(k as u32);
    }

    let mut l3_num = l1_num + l2_num;
    let mut l3_vals = vec![];
    if l3_num == 0 {
        l3_vals = vec![0];
    } else {
        while l3_num > 0 {
            let r = l3_num % 10;
            l3_vals.insert(0, r);
            l3_num /= 10;
        }
    }
    let mut l3: Option<Box<ListNode>> = None;
    for item3 in l3_vals {
        l3 = Some(Box::new(ListNode {
            val: (item3 as i32),
            next: l3.take(),
        }));
    }

    return l3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l3 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));
        assert_eq!(l3, add_two_numbers(l1, l2));
    }
}
