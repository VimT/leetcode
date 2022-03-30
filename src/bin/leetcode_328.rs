//! 奇偶链表

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_list1 = Some(Box::new(ListNode::new(0)));
    let mut dummy_list2 = Some(Box::new(ListNode::new(0)));
    let mut tail_ptr1 = &mut dummy_list1;
    let mut tail_ptr2 = &mut dummy_list2;

    let mut is_odd = true; // first one is odd
    let mut cur = head;
    while let Some(mut cur_inner) = cur {
        cur = cur_inner.next.take();
        if is_odd {
            tail_ptr1.as_mut().unwrap().next = Some(cur_inner);
            tail_ptr1 = &mut tail_ptr1.as_mut().unwrap().next;
        } else {
            tail_ptr2.as_mut().unwrap().next = Some(cur_inner);
            tail_ptr2 = &mut tail_ptr2.as_mut().unwrap().next;
        }
        is_odd = !is_odd;
    }
    if let Some(node2) = dummy_list2.as_mut().unwrap().next.take() {
        tail_ptr1.as_mut().unwrap().next = Some(node2);
    }
    dummy_list1.unwrap().next
}


fn main() {
    assert_eq!(odd_even_list(link![1,2,3,4,5]), link![1,3,5,2,4]);
    assert_eq!(odd_even_list(link![2,1,3,5,6,4,7]), link![2,3,6,7,1,5,4]);
}
