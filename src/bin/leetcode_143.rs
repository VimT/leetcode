//! 重排链表

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    if head.is_none() { return; }
    let mut p: *mut _ = head;
    let mut len = 0;
    unsafe {
        while p.as_ref().unwrap().is_some() {
            p = &mut p.as_mut().unwrap().as_mut().unwrap().next;
            len += 1;
        }
        let skip = (len - 1) / 2;
        p = head;
        for _ in 0..skip {
            p = &mut p.as_mut().unwrap().as_mut().unwrap().next;
        }
        let mut p2 = p.as_mut().unwrap().as_mut().unwrap().next.take();

        // reverse
        let mut pre = None;

        while p2.is_some() {
            let next = p2.as_mut().unwrap().next.take();
            p2.as_mut().unwrap().next = pre.take();
            pre = p2;
            p2 = next;
        }

        // insert
        let mut p2 = pre;
        let mut p1: *mut _ = head;
        while p1.as_ref().unwrap().is_some() && p2.is_some() {
            let mut p1n = p1.as_mut().unwrap().as_mut().unwrap().next.take();
            let p2n = p2.as_mut().unwrap().next.take();
            p2.as_mut().unwrap().next = p1n.take();
            p1.as_mut().unwrap().as_mut().unwrap().next = p2.take();
            p1 = &mut p1.as_mut().unwrap().as_mut().unwrap().next.as_mut().unwrap().next;
            p2 = p2n;
        }
    }
}

fn main() {
    fn test(func: fn(head: &mut Option<Box<ListNode>>)) {
        let help = |mut head: Option<Box<ListNode>>| {
            func(&mut head);
            head
        };
        assert_eq!(help(link![1, 2, 3, 4]), link![1, 4, 2, 3]);
        assert_eq!(help(link![1, 2, 3, 4, 5]), link![1, 5, 2, 4, 3]);
    }
    test(reorder_list);
}
