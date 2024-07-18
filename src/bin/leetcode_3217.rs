//! 从链表中移除在数组中存在的节点

use std::collections::HashSet;

use leetcode::linknode::ListNode;

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let nums: HashSet<i32> = nums.into_iter().collect();
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut p = &mut dummy;
    while let Some(node) = p.as_mut().unwrap().next.as_mut() {
        if nums.contains(&node.val) {
            p.as_mut().unwrap().next = node.next.take();
        } else {
            p = &mut p.as_mut().unwrap().next;
        }
    }
    dummy.unwrap().next
}

fn main() {
    use leetcode::link;
    fn test(func: fn(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
        assert_eq!(func(vec![1, 2, 3], link![1,2,3,4,5]), link![4,5]);
        assert_eq!(func(vec![1], link![1,2,1,2,1,2]), link![2,2,2]);
        assert_eq!(func(vec![5], link![1,2,3,4]), link![1,2,3,4]);
    }
    test(modified_list);
}
