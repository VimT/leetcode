//! 移除链表元素

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut fake = Some(Box::new(ListNode::new(0)));
    fake.as_mut().unwrap().next = head;
    let mut p = &mut fake;
    while p.is_some() && p.as_ref().unwrap().next.is_some() {
        while p.as_ref().unwrap().next.is_some() && p.as_ref().unwrap().next.as_ref().unwrap().val == val {
            let tmp = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = tmp;
        }
        p = &mut p.as_mut().unwrap().next;
    }
    fake.unwrap().next
}

fn main() {
    assert_eq!(remove_elements(link![7, 7, 7, 7], 7), link![]);
    assert_eq!(remove_elements(link![1, 2, 6, 3, 4, 5, 6], 6), link![1, 2, 3, 4, 5]);
    assert_eq!(remove_elements(link![], 1), link![]);
}
