//! 合并两个有序链表

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut p = &mut head;
    while let (Some(x), Some(y)) = (&l1, &l2) {
        if x.val < y.val {
            // 注意所有权转移，这个顺序不能变，先设置p.next
            p.next = l1;
            // 自动解box
            p = p.next.as_mut().unwrap();
            // 使用take进行Option内容的所有权转移，take 需要&mut， 只有p.next持有当前&mut
            l1 = p.next.take();
        } else {
            p.next = l2;
            p = p.next.as_mut().unwrap();
            l2 = p.next.take();
        }
    }
    p.next = if l1.is_some() { l1 } else { l2 };
    head.next
}

pub fn merge_two_lists2(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut p = &mut dummy;
    while l1.is_some() && l2.is_some() {
        // as_deref_mut 直接得到Box的内部值
        let (p1, p2) = (l1.as_deref_mut().unwrap(), l2.as_deref_mut().unwrap());
        if p1.val < p2.val {
            let next = p1.next.take();
            p.next = l1.take();
            l1 = next;
        } else {
            let next = p2.next.take();
            p.next = l2.take();
            l2 = next;
        }
        p = p.next.as_deref_mut().unwrap();
    }
    p.next = l1.or(l2);
    dummy.next
}

fn main() {
    fn test(func: fn(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
        assert_eq!(func(link![1, 2, 4], link![1, 3, 4]), link![1, 1, 2, 3, 4, 4]);
        assert_eq!(func(link![], link![]), link![]);
        assert_eq!(func(link![], link![0]), link![0]);
    }
    test(merge_two_lists);
    test(merge_two_lists2);
}
