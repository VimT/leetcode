//! 链表组件


use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
    let mut map = vec![0; 10001];
    for num in nums {
        map[num as usize] += 1;
    }
    let mut p = &head;
    let mut result = 0;
    let mut lx = 0;
    while !p.is_none() {
        let val = p.as_ref().unwrap().val;
        if map[val as usize] > 0 {
            lx += 1;
            map[val as usize] -= 1;
        } else {
            if lx > 0 {
                result += 1;
                lx = 0;
            }
        }
        let nxt = &p.as_ref().unwrap().next;
        p = nxt;
    }
    if lx > 0 { result += 1; }
    result
}

fn main() {
    assert_eq!(num_components(link![0, 1, 2, 3], vec![0, 1, 3]), 2);
    assert_eq!(num_components(link![0, 1, 2, 3, 4], vec![0, 3, 1, 4]), 2);
}
