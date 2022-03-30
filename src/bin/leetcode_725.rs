//! 分隔链表

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut p = &head;
    let mut len = 0;
    while p.is_some() {
        let nxt = &p.as_ref().unwrap().next;
        p = nxt;
        len += 1;
    }
    let avg_len = len as i32 / k;
    fn get_len(mut head: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if k == 0 { return (None, head); }
        let mut p = &mut head;
        for _ in 0..k - 1 {
            if p.is_none() { break; }
            let nxt = &mut p.as_mut().unwrap().next;
            p = nxt;
        }
        let left = p.as_mut().map(|x| x.next.take()).unwrap_or(None);
        (head, left)
    }
    let mut result = Vec::with_capacity(k as usize);
    let more = len as i32 % k;
    for _ in 0..more {
        let (section, left) = get_len(head, avg_len + 1);
        result.push(section);
        head = left;
    }
    for _ in 0..k - more {
        let (section, left) = get_len(head, avg_len);
        result.push(section);
        head = left;
    }
    result
}

fn main() {
    assert_eq!(split_list_to_parts(link![1,2,3], 5), vec![link![1], link![2], link![3], link![], link![]]);
    assert_eq!(split_list_to_parts(link![1,2,3,4,5,6,7,8,9,10], 3), vec![link![1,2,3,4], link![5,6,7], link![8,9,10]]);
}
