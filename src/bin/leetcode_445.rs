//! 两数相加 II

use leetcode::link;
use leetcode::linknode::{link_to_vec, ListNode};

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    #[inline]
    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        let mut next;
        while cur.is_some() {
            next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = pre.take();
            pre = cur.take();
            cur = next.take();
        }
        pre
    }
    let mut l1 = reverse(l1);
    let mut l2 = reverse(l2);
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut carry = 0;
    let mut p = &mut head;
    while l1.is_some() || l2.is_some() {
        let val = l1.as_ref().map(|x| x.val).unwrap_or(0) + l2.as_ref().map(|x| x.val).unwrap_or(0) + carry;
        p.as_mut().unwrap().next = Some(Box::new(ListNode::new(val % 10)));
        carry = val / 10;
        p = &mut p.as_mut().unwrap().next;
        if l1.is_some() {
            let tmp = l1.unwrap().next.take();
            l1 = tmp;
        }
        if l2.is_some() {
            let tmp = l2.unwrap().next.take();
            l2 = tmp;
        }
    }
    if carry > 0 {
        p.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
    }
    reverse(head.as_mut().unwrap().next.take())
}

fn main() {
    assert_eq!(link_to_vec(add_two_numbers(link![5], link![5])), vec![1, 0]);
    assert_eq!(link_to_vec(add_two_numbers(link![7, 2, 4, 3], link![5, 6, 4])), vec![7, 8, 0, 7]);
    assert_eq!(link_to_vec(add_two_numbers(link![2, 4, 3], link![5, 6, 4])), vec![8, 0, 7]);
    assert_eq!(link_to_vec(add_two_numbers(link![0], link![0])), vec![0]);
}
