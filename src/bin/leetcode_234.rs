//! 回文链表

use leetcode::linknode::ListNode;
use leetcode::link;

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut s = vec![];
    let mut p = &head;
    let mut len = 0;
    while p.is_some() {
        p = &p.as_ref().unwrap().next;
        len += 1;
    }
    p = &head;
    for _ in 0..len / 2 {
        s.push(p.as_ref().unwrap().val);
        p = &p.as_ref().unwrap().next;
    }
    if len & 1 == 1 {
        p = &p.as_ref().unwrap().next;
    }
    for _ in 0..len / 2 {
        if p.as_ref().unwrap().val != s.pop().unwrap() {
            return false;
        }
        p = &p.as_ref().unwrap().next;
    }
    true
}

fn main() {
    fn test(func: fn(head: Option<Box<ListNode>>) -> bool) {
        assert_eq!(func(link![1,2,2,1]), true);
        assert_eq!(func(link![1,2]), false);
        assert_eq!(func(link![1]), true);
        assert_eq!(func(link![1,2,1]), true);
    }
    test(is_palindrome);
}
