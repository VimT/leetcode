//! 删除排序链表中的重复元素 II


use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fake = Some(Box::new(ListNode::new(0)));
    fake.as_mut().unwrap().next = head;
    let mut pre = &mut fake;
    while pre.as_ref().unwrap().next.is_some() {
        let mut cur = &mut pre.as_mut().unwrap().next;
        let mut same = false;
        while cur.as_ref().unwrap().next.is_some() && cur.as_ref().unwrap().val == cur.as_ref().unwrap().next.as_ref().unwrap().val {
            same = true;
            cur = &mut cur.as_mut().unwrap().next;
        }
        if same {
            pre.as_mut().unwrap().next = cur.as_mut().unwrap().next.take();
        } else {
            // pre.as_mut().unwrap().next = cur.take();
            pre = &mut pre.as_mut().unwrap().next;
        }
    }
    fake.as_mut().unwrap().next.take()
}

fn main() {
    assert_eq!(delete_duplicates(link![1, 2, 3, 3, 4, 4, 5]), link![1, 2, 5]);
    assert_eq!(delete_duplicates(link![1, 1, 1, 2, 3]), link![2, 3]);
}
