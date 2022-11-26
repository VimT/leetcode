//! 反转偶数长度组的节点

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn reverse_even_length_groups(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unsafe fn reverse(head: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
        if head.is_none() { return head; }
        let head = Box::into_raw(head.unwrap());
        let mut pre = std::ptr::null_mut();
        let mut next;
        let mut cur = head;
        for _ in 0..len {
            if cur.is_null() {
                break;
            }
            next = match (*cur).next.take() {
                None => { std::ptr::null_mut() }
                Some(v) => { Box::into_raw(v) }
            };
            (*cur).next = Some(Box::from_raw(pre));
            pre = cur;
            cur = next;
        }
        (*head).next = Some(Box::from_raw(cur));
        Some(Box::from_raw(pre))
    }
    let mut cur_length = 1;
    let mut fake = Some(Box::new(ListNode::new(0)));
    fake.as_mut().unwrap().next = head.take();
    let mut p = &mut fake;
    let mut total_len = 0;
    while p.is_some() {
        p = &mut p.as_mut().unwrap().next;
        total_len += 1;
    }
    p = &mut fake;
    let mut cur = 1;
    loop {
        if (total_len - cur >= cur_length && (cur_length & 1 == 0)) || ((total_len - cur) < cur_length && (total_len - cur) & 1 == 0) {
            unsafe { p.as_mut().unwrap().next = reverse(p.as_mut().unwrap().next.take(), cur_length); }
        }
        for _ in 0..cur_length {
            p = &mut p.as_mut().unwrap().next;
            if p.is_none() {
                return fake.as_mut().unwrap().next.take();
            }
            cur += 1;
        }
        cur_length += 1;
    }
}

fn main() {
    assert_eq!(reverse_even_length_groups(link![5,2,6,3,9,1,7,3,8,4]), link![5,6,2,3,9,1,4,8,3,7]);
    assert_eq!(reverse_even_length_groups(link![1,1,0,6]), link![1,0,1,6]);
    assert_eq!(reverse_even_length_groups(link![1,1,0,6,5]), link![1,0,1,5,6]);
}
