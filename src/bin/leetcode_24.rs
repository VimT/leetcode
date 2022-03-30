//! 两两交换链表中的节点

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    dummy.next = head;
    let mut p = &mut dummy;
    while p.next.is_some() {
        let mut left = p.next.take();
        let mut right = left.as_mut().unwrap().next.take();
        if right.is_none() {
            p.next = left.take();
            break;
        }
        let mut next = right.as_mut().unwrap().next.take();
        p.next = right.take();
        p = p.next.as_mut().unwrap();
        p.next = left.take();
        p = p.next.as_mut().unwrap();
        p.next = next.take();
    }
    dummy.next
}

fn main() {
    assert_eq!(swap_pairs(link![1, 2, 3, 4]), link![2, 1, 4, 3]);
    assert_eq!(swap_pairs(link![]), link![]);
    assert_eq!(swap_pairs(link![1]), link![1]);
}
