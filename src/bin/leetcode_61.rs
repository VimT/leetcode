//! 旋转链表


use leetcode::link;
use leetcode::linknode::ListNode;

pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut p = &mut head;
    while p.is_some() {
        len += 1;
        p = &mut p.as_mut().unwrap().next;
    }
    if len == 0 { return head; }
    let k = k % len;
    if k == 0 { return head; }
    let forward = len - k;
    p = &mut head;
    for _ in 0..forward - 1 {
        p = &mut p.as_mut().unwrap().next;
    }
    let mut tmp = p.as_mut().unwrap().next.take();
    let mut last = &mut tmp;
    for _ in 0..k - 1 {
        last = &mut last.as_mut().unwrap().next;
    }
    last.as_mut().unwrap().next = head;
    tmp
}


fn main() {
    assert_eq!(rotate_right(link![1, 2, 3, 4, 5], 2), link![4, 5, 1, 2, 3]);
    assert_eq!(rotate_right(link![0, 1, 2], 4), link![2, 0, 1]);
}
