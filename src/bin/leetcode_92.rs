//! 反转链表 II

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn reverse_between(mut head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let mut fake = Some(Box::new(ListNode::new(0)));
    fake.as_mut().unwrap().next = head.take();
    let mut before = &mut fake;
    for _ in 1..m {
        before = &mut before.as_mut().unwrap().next;
    }
    let mut pre = None;
    let mut cur = before.as_mut().unwrap().next.take();
    let mut next;

    for _ in m..=n {
        next = cur.as_mut().unwrap().next.take();
        cur.as_mut().unwrap().next = pre.take();
        pre = cur.take();
        cur = next.take();
    }
    before.as_mut().unwrap().next = pre.take();
    for _ in m..=n {
        before = &mut before.as_mut().unwrap().next;
    }
    before.as_mut().unwrap().next = cur.take();
    fake.as_mut().unwrap().next.take()
}

fn main() {
    assert_eq!(reverse_between(link![1, 2, 3, 4, 5], 2, 4), link![1, 4, 3, 2, 5]);
    assert_eq!(reverse_between(link![5], 1, 1), link![5]);
}
