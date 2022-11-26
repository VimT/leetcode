//! 删除链表的中间节点

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut p = &head;
    while p.is_some() {
        len += 1;
        p = &p.as_deref().unwrap().next;
    }
    if len == 1 { return None; }
    let target = len / 2;
    let mut p = &mut head;
    for _ in 0..target - 1 {
        p = &mut p.as_mut().unwrap().next;
    }
    let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
    p.as_mut().unwrap().next = next;
    head
}

fn main() {
    assert_eq!(delete_middle(link![1,3,4,7,1,2,6]), link![1,3,4,1,2,6]);
    assert_eq!(delete_middle(link![1,2,3,4]), link![1,2,4]);
    assert_eq!(delete_middle(link![2,1]), link![2]);
}
