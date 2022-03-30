//! 反转链表

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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


fn main() {
    assert_eq!(reverse_list(link![1, 2, 3, 4, 5]), link![5, 4, 3, 2, 1]);
    assert_eq!(reverse_list(link![1, 2]), link![2, 1]);
    assert_eq!(reverse_list(link![]), link![]);
}
