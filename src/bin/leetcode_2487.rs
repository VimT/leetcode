//! 从链表中移除节点

use leetcode::linknode::{ListNode, vec_to_link};
use leetcode::link;

/// 单调栈
pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut s = vec![];
    let mut p = &head;
    while p.is_some() {
        while !s.is_empty() && *s.last().unwrap() < p.as_ref().unwrap().val {
            s.pop().unwrap();
        }
        s.push(p.as_ref().unwrap().val);
        p = &p.as_ref().unwrap().next;
    }
    vec_to_link(s)
}


/// 递归
pub fn remove_nodes2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn inner(node: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
        if node.is_none() { return (None, 0); }
        let val = node.as_ref().unwrap().val;
        let (next, max) = inner(node.unwrap().next);
        if max > val { return (next, max); }
        let mut link = ListNode::new(val);
        link.next = next;
        (Some(Box::new(link)), val)
    }
    inner(head).0
}


fn main() {
    fn test(func: fn(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
        assert_eq!(func(link![5,2,13,3,8]), link![13,8]);
        assert_eq!(func(link![1,1,1,1]), link![1,1,1,1]);
    }
    test(remove_nodes);
    test(remove_nodes2);
}
