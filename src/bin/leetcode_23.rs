//! 合并K个升序链表

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::with_capacity(lists.len());
    let mut head = Box::new(ListNode::new(0));
    let mut p = &mut head;
    for i in 0..lists.len() {
        if lists[i].is_some() {
            let mut n = lists[i].take().unwrap();
            lists[i] = n.next.take();
            heap.push((Reverse(n), i));
        }
    }

    while !heap.is_empty() {
        let (Reverse(n), i) = heap.pop().unwrap();
        p.next = Some(n);
        p = p.next.as_mut().unwrap();
        if lists[i].is_some() {
            let mut ni = lists[i].take().unwrap();
            lists[i] = ni.next.take();
            heap.push((Reverse(ni), i));
        }
    }

    head.next
}


fn main() {
    assert_eq!(merge_k_lists(vec![link![1, 4, 5], link![1, 3, 4], link![2, 6]]), link![1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(merge_k_lists(vec![]), link![]);
    assert_eq!(merge_k_lists(vec![link![]]), link![]);
}
