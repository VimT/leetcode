//! 从链表中删去总和值为零的连续节点

use std::collections::HashMap;

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut arr = vec![];
    let mut p = &head;
    while let Some(pp) = p {
        arr.push(pp.val);
        p = &pp.next;
    }
    loop {
        let mut m = HashMap::new();
        m.insert(0, 0);
        let mut cur_sum = 0;
        let mut ok = true;
        for i in 0..arr.len() {
            cur_sum += arr[i];
            if let Some(pre_idx) = m.get(&cur_sum) {
                arr.drain(*pre_idx..=i);
                ok = false;
                break;
            }
            m.insert(cur_sum, i + 1);
        }
        if ok {
            break;
        }
    }
    vec_to_link(arr)
}

/// 直接使用链表做，不用转数组
pub fn remove_zero_sum_sublists2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut map = HashMap::new();
    let mut ptr = &mut head;
    let mut sum = 0;
    let mut index = 0;
    while let Some(curr) = ptr {
        sum += curr.val;
        map.insert(sum, index);
        ptr = &mut curr.next;
        index += 1;
    }
    sum = 0;
    index = -1;
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    ptr = &mut dummy;

    while let Some(curr) = ptr {
        sum += curr.val;
        if let Some(&next) = map.get(&sum) {
            if next > index {
                let mut tail = curr.next.take();
                for _ in 0..next - index {
                    if let Some(t) = tail {
                        tail = t.next;
                    }
                }
                index += next - index;
                curr.next = tail;
            }
        }
        index += 1;
        ptr = &mut curr.next;
    }

    dummy.unwrap().next
}

fn main() {
    fn test(func: fn(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
        assert_eq!(func(link![-1,1,-2,-1]), link![-2,-1]);
        assert_eq!(func(link![2,0]), link![2]);
        assert_eq!(func(link![0,2]), link![2]);
        assert_eq!(func(link![1,-1]), link![]);
        assert_eq!(func(link![1,2,-3,3,1]), link![3,1]);
        assert_eq!(func(link![1,2,3,-3,4]), link![1,2,4]);
        assert_eq!(func(link![1,2,3,-3,-2]), link![1]);
    }
    test(remove_zero_sum_sublists);
    test(remove_zero_sum_sublists2);
}
