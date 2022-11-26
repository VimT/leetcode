//! 链表的中间结点

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p = &head;
    let mut len = 0;
    while p.is_some() {
        let nxt = &p.as_ref().unwrap().next;
        len += 1;
        p = nxt;
    }
    if len == 1 { return head; }
    let mid = len / 2;
    let mut p = &mut head;
    for _ in 0..mid - 1 {
        let nxt = &mut p.as_mut().unwrap().next;
        p = nxt;
    }
    p.as_mut().unwrap().next.take()
}

fn main() {
    assert_eq!(middle_node(link![1, 2, 3, 4, 5]), link![3, 4, 5]);
    assert_eq!(middle_node(link![1, 2, 3, 4, 5, 6]), link![4, 5, 6]);
    assert_eq!(middle_node(link![1]), link![1]);
}
