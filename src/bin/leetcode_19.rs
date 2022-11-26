//! 删除链表的倒数第 N 个结点

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if n == 0 { return head; }
    let mut p1: *const _ = &head;
    let mut p2 = head.as_mut().unwrap();
    for _ in 0..n {
        p1 = unsafe { &(*p1).as_ref().unwrap().next };
    }
    if unsafe { &*p1 }.is_none() {
        return head.unwrap().next;
    }
    while let Some(_) = unsafe { (&*p1).as_ref().unwrap().next.as_ref() } {
        p1 = unsafe { &(*p1).as_ref().unwrap().next };
        p2 = p2.next.as_mut().unwrap();
    }
    p2.next = p2.next.take().unwrap().next;
    head
}


fn main() {
    assert_eq!(remove_nth_from_end(link![1, 2, 3, 4, 5], 2), link![1, 2, 3, 5]);
    assert_eq!(remove_nth_from_end(link![1], 1), link![]);
    assert_eq!(remove_nth_from_end(link![1, 2], 1), link![1]);
}
