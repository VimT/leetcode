//! K 个一组翻转链表

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

/// 黑科技
pub fn reverse_k_group_black_tech(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 || head.is_none() { return head; }
    let mut p: *mut _ = &mut head;
    let mut last = [std::ptr::null_mut(); 2];
    loop {
        unsafe {
            let start = p;
            let mut cur = p;
            for i in 0..k {
                cur = if let Some(cur) = (*cur).as_mut() {
                    &mut cur.next
                } else {
                    return head;
                };
                if i == 0 { p = cur }
                if i >= k - 2 { last[(i - k + 2) as usize] = cur }
            }
            cur = start;
            let mut i = 0;
            for _ in 0..k - 1 {
                let next = &mut (*cur).as_mut().unwrap().next;
                // *cur = *last[i]
                std::ptr::swap_nonoverlapping(cur, last[i], 1);
                cur = next;
                i ^= 1;
            }
            if i == 1 { std::ptr::swap_nonoverlapping(last[0], last[1], 1) }
        }
    }
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut p = head;
        let mut next;
        while p.is_some() {
            next = p.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = pre.take();
            pre = p.take();
            p = next.take()
        }
        pre
    }
    if k == 1 { return head; }
    let mut dummy = Some(Box::new(ListNode::new(0)));
    dummy.as_mut().unwrap().next = head;
    let mut pre: *mut _ = &mut dummy;
    let mut end: *mut _ = &mut dummy;

    unsafe {
        while (*end).as_ref().unwrap().next.is_some() {
            for _ in 0..k {
                if (*end).is_none() { break; }
                end = &mut (*end).as_mut().unwrap().next;
            }
            if (*end).is_none() { break; }
            // 执行这一步之后，pre的指针变成了 pre.next ，迷
            let mut start = (*pre).as_mut().unwrap().next.take();
            // 所以重新从头节点遍历一下
            pre = &mut dummy;
            while pre.as_ref().unwrap().as_ref().unwrap().next.is_some() {
                pre = &mut (*pre).as_mut().unwrap().next;
            }

            let mut next = (*end).as_mut().unwrap().next.take();
            let startp: *mut _ = &mut start;
            (*end).as_mut().unwrap().next = None;
            (*pre).as_mut().unwrap().next = reverse(start).take();
            (*startp).as_mut().unwrap().next = next.take();
            pre = startp;
            end = pre;
        }
    }
    dummy.as_mut().unwrap().next.take()
}

fn main() {
    fn test(func: fn(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>) {
        assert_eq!(func(link![1, 2, 3, 4, 5], 2), link![2, 1, 4, 3, 5]);
        assert_eq!(func(link![1, 2, 3, 4, 5], 3), link![3, 2, 1, 4, 5]);
    }
    test(reverse_k_group);
    test(reverse_k_group_black_tech);
}
