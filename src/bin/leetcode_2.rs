//! 两数相加

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut head = ListNode::new(0);
    let mut cur = &mut head;
    while l1.is_some() || l2.is_some() {
        let n = l1.as_ref().map_or(0, |x| x.val) + l2.as_ref().map_or(0, |x| x.val) + carry;
        let this = ListNode::new(n % 10);
        carry = n / 10;
        cur.next = Some(Box::new(this));
        cur = cur.next.as_mut().unwrap();
        l1 = l1.map_or(None, |mut x| x.next.take());
        l2 = l2.map_or(None, |mut x| x.next.take());
    }
    if carry == 1 {
        cur.next = Some(Box::new(ListNode::new(1)));
    }

    head.next.take()
}

fn main() {
    assert_eq!(add_two_numbers(link![2, 4, 3], link![5, 6, 4]), link![7, 0, 8]);
    assert_eq!(add_two_numbers(link![0], link![0]), link![0]);
    assert_eq!(add_two_numbers(link![9, 9, 9, 9, 9, 9, 9], link![9, 9, 9, 9]), link![8, 9, 9, 9, 0, 0, 0, 1]);
}
