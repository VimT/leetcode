//! 合并零之间的节点


use leetcode::link;
use leetcode::linknode::ListNode;

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = ListNode::new(0);
    let mut rp = &mut result;
    let mut p = &head.as_ref().unwrap().next;
    let mut cur_sum = 0;
    while p.is_some() {
        cur_sum += p.as_ref().unwrap().val;
        if p.as_ref().unwrap().val == 0 {
            rp.next = Some(Box::new(ListNode::new(cur_sum)));
            cur_sum = 0;
            rp = rp.next.as_mut().unwrap();
        }
        p = &p.as_ref().unwrap().next;
    }
    result.next.take()
}

fn main() {
    assert_eq!(merge_nodes(link![0, 3, 1, 0, 4, 5, 2, 0]), link![4, 11]);
    assert_eq!(merge_nodes(link![0, 1, 0, 3, 0, 2, 2, 0]), link![1, 3, 4]);
}
