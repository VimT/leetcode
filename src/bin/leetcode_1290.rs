//! 二进制链表转整数

use leetcode::linknode::ListNode;
use leetcode::link;

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut p = &head;
    let mut num = 0;
    while let Some(node) = p {
        num = num << 1 | node.val;
        p = &node.next;
    }
    num
}

fn main() {
    fn test(func: fn(head: Option<Box<ListNode>>) -> i32) {
        assert_eq!(func(link![1,0,1]), 5);
        assert_eq!(func(link![0]), 0);
    }
    test(get_decimal_value);
}
