//! 链表最大孪生和


use leetcode::link;
use leetcode::linknode::ListNode;

pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut len = 0;
    let mut p = &head;
    while p.is_some() {
        len += 1;
        p = &p.as_ref().unwrap().next;
    }
    let mut result = 0;
    let mut prev = vec![0; len / 2];
    p = &head;
    for i in 0..len / 2 {
        prev[i] = p.as_ref().unwrap().val;
        p = &p.as_ref().unwrap().next;
    }
    for _ in 0..len / 2 {
        let val = p.as_ref().unwrap().val;
        result = result.max(val + prev.pop().unwrap());
        p = &p.as_ref().unwrap().next;
    }
    result
}

fn main() {
    assert_eq!(pair_sum(link![5, 4, 2, 1]), 6);
    assert_eq!(pair_sum(link![4, 2, 2, 3]), 7);
    assert_eq!(pair_sum(link![1, 100000]), 100001);
}
