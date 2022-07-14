//! 相交链表

use std::ptr::null_mut;

/// 默认的ListNode不能表达公共后缀，所以只能用unsafe+*mut指针造一个了
pub struct ListNode {
    val: i32,
    next: *mut ListNode,
}

impl ListNode {
    fn new_one(val: i32) -> *mut ListNode {
        Box::into_raw(Box::new(ListNode { val, next: null_mut() }))
    }
    unsafe fn new(nodes: Vec<i32>) -> *mut ListNode {
        let dummy = ListNode::new_one(0);
        let mut p = dummy;
        for node in nodes {
            (*p).next = ListNode::new_one(node);
            p = (*p).next;
        }
        (*dummy).next
    }
}

/// 两链表找公共后缀（LCA）
pub unsafe fn get_intersection_node(head_a: *mut ListNode, head_b: *mut ListNode) -> *mut ListNode {
    let mut p1 = head_a;
    let mut p2 = head_b;
    while p1 != p2 {
        p1 = if p1.is_null() { head_b } else { (*p1).next };
        p2 = if p2.is_null() { head_a } else { (*p2).next };
    }
    p1
}

fn main() {
    unsafe fn help(a: Vec<i32>, b: Vec<i32>, skip_a: usize, skip_b: usize) -> *mut ListNode {
        assert_eq!(a[skip_a..], b[skip_b..]);
        let common = ListNode::new(a[skip_a..].to_vec());
        let al = ListNode::new(a[..skip_a].to_vec());
        let bl = ListNode::new(b[..skip_b].to_vec());
        (*al).next = common;
        (*bl).next = common;
        get_intersection_node(al, bl)
    }
    unsafe {
        assert_eq!(help(vec![4, 1, 8, 4, 5], vec![5, 6, 1, 8, 4, 5], 2, 3).as_ref().unwrap().val, 8);
        assert_eq!(help(vec![1, 9, 1, 2, 4], vec![3, 2, 4], 3, 1).as_ref().unwrap().val, 2);
        assert_eq!(help(vec![2, 6, 4], vec![1, 5], 3, 2).is_null(), true);
        assert_eq!(help(vec![1], vec![3], 1, 1).is_null(), true);
    }
}
