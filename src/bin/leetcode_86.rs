//! 分隔链表

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut small = Some(Box::new(ListNode::new(0)));
    let mut big = Some(Box::new(ListNode::new(0)));
    let mut p1 = &mut small;
    let mut p2 = &mut big;
    let mut p = &mut head;

    while p.is_some() {
        if p.as_ref().unwrap().val < x {
            p1.as_mut().unwrap().next = p.take();
            p1 = &mut p1.as_mut().unwrap().next;
            p = &mut p1.as_mut().unwrap().next;
        } else {
            p2.as_mut().unwrap().next = p.take();
            p2 = &mut p2.as_mut().unwrap().next;
            p = &mut p2.as_mut().unwrap().next;
        }
    }
    if big.as_ref().unwrap().next.is_some() {
        p1.as_mut().unwrap().next = big.as_mut().unwrap().next.take();
    }
    small.as_mut().unwrap().next.take()
}


fn main() {
    assert_eq!(partition(link![1, 4, 3, 2, 5, 2], 3), link![1, 2, 2, 4, 3, 5]);
    assert_eq!(partition(link![2, 1], 2), link![1, 2]);
}
