//! 对链表进行插入排序

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() { return None; }
    let mut head = head.unwrap();
    let mut cur = head.next.take();
    let mut ans = ListNode::new(0);
    ans.next = Some(Box::new(ListNode::new(head.val)));
    let mut result = Some(Box::new(ans));
    while cur.is_some() {
        let mut pre = result.as_mut().unwrap();
        let val = cur.as_ref().unwrap().val;
        while pre.next.is_some() {
            if pre.next.as_ref().unwrap().val >= val {
                break;
            }
            pre = pre.next.as_mut().unwrap();
        }
        let tmp = pre.next.take();
        let mut data = ListNode::new(val);
        data.next = tmp;
        pre.next = Some(Box::new(data));
        cur = cur.unwrap().next;
    }
    return result.unwrap().next;
}

fn main() {
    assert_eq!(insertion_sort_list(link![4,2,1,3]), link![1,2,3,4]);
    assert_eq!(insertion_sort_list(link![-1,5,3,4,0]), link![-1,0,3,4,5]);
}
