//! 给单链表加一

use leetcode::linknode::{ListNode, vec_to_link};
use leetcode::link;

pub fn plus_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sentinel = Some(Box::new(ListNode::new(0)));
    sentinel.as_mut().unwrap().next = head;
    let mut not_nine = sentinel.as_mut().map(|x| x as *mut Box<ListNode>);
    let mut p = &sentinel;
    while let Some(pp) = p {
        if pp.val != 9 {
            not_nine = Some(pp as *const _ as *mut _);
        }
        p = &pp.next;
    }

    unsafe {
        (*not_nine.unwrap()).val += 1;
        not_nine = not_nine.and_then(|x| (*x).next.as_mut().map(|x| x as *mut _));

        while let Some(nn) = not_nine {
            (*nn).val = 0;
            not_nine = (*nn).next.as_mut().map(|x| x as *mut _);
        }
    }
    if sentinel.as_ref().unwrap().val > 0 { sentinel } else { sentinel.as_mut().unwrap().next.take() }
}

fn main() {
    fn test(func: fn(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
        assert_eq!(func(link![8,9,9]), link![9,0,0]);
        assert_eq!(func(link![1,2,3]), link![1,2,4]);
        assert_eq!(func(link![0]), link![1]);
        assert_eq!(func(link![9,9,9]), link![1,0,0,0]);
    }
    test(plus_one);
}
