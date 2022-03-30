//! 链表中的下一个更大节点

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut s: Vec<(i32, usize)> = vec![];
    let mut result = vec![];
    let mut p = &head;
    while let Some(n) = p {
        let val = n.val;
        while !s.is_empty() && val > s.last().unwrap().0 {
            let (_, idx) = s.pop().unwrap();
            result[idx] = val;
        }
        s.push((val, result.len()));
        p = &n.next;
        result.push(0);
    }
    result
}

fn main() {
    assert_eq!(next_larger_nodes(link![2, 1, 5]), vec![5, 5, 0]);
    assert_eq!(next_larger_nodes(link![2, 7, 4, 3, 5]), vec![7, 0, 5, 5, 0]);
}
