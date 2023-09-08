//! 在链表中插入最大公约数

use leetcode::gcd::gcd;
use leetcode::linknode::ListNode;

pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p = head.as_ref().unwrap();
    let mut result = Box::new(ListNode::new(0));
    let mut wp = &mut result;
    while p.next.is_some() {
        let next_val = p.next.as_ref().unwrap().val;
        wp.next = Some(Box::new(ListNode::new(p.val)));
        wp = wp.next.as_mut().unwrap();
        wp.next = Some(Box::new(ListNode::new(gcd(p.val, next_val))));
        wp = wp.next.as_mut().unwrap();
        p = p.next.as_ref().unwrap();
    }
    wp.next = Some(Box::new(ListNode::new(p.val)));
    result.next
}

/// 原地修改
pub fn insert_greatest_common_divisors2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p = head.as_mut().unwrap();
    while p.next.is_some() {
        let mut node = Box::new(ListNode::new(gcd(p.val, p.next.as_ref().unwrap().val)));
        node.next = p.next.take();
        p.next = Some(node);
        p = p.next.as_mut().unwrap().next.as_mut().unwrap();
    }
    head
}

fn main() {
    use leetcode::link;
    fn test(func: fn(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
        assert_eq!(func(link![18,6,10,3]), link![18,6,6,2,10,1,3]);
        assert_eq!(func(link![7]), link![7]);
    }
    test(insert_greatest_common_divisors);
    test(insert_greatest_common_divisors2);
}
