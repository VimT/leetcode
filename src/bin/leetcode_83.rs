//! 删除排序链表中的重复元素

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() { return head; }
    let mut p = &mut head;
    while p.as_ref().unwrap().next.is_some() {
        if p.as_ref().unwrap().val == p.as_ref().unwrap().next.as_ref().unwrap().val {
            p.as_mut().unwrap().next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else {
            p = &mut p.as_mut().unwrap().next;
        }
    }
    head
}

fn main() {
    assert_eq!(delete_duplicates(link![1, 1, 2]), link![1, 2]);
    assert_eq!(delete_duplicates(link![1, 1, 2, 3, 3]), link![1, 2, 3]);
}
