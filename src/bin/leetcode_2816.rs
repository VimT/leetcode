//! 翻倍以链表形式表示的数字


use leetcode::linknode::ListNode;

pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.as_ref().unwrap().val >= 5 {
        let mut new_head = Box::new(ListNode::new(0));
        new_head.next = head;
        head = Some(new_head);
    }
    let mut cur = head.as_mut();
    while let Some(p) = cur {
        p.val = p.val * 2 % 10;
        if p.next.is_some() && p.next.as_ref().unwrap().val >= 5 {
            p.val += 1;
        }
        cur = p.next.as_mut();
    }
    head
}

fn main() {
    use leetcode::link;
    fn test(func: fn(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
        assert_eq!(func(link![1,8,9]), link![3,7,8]);
        assert_eq!(func(link![9,9,9]), link![1,9,9,8]);
    }
    test(double_it);
}
