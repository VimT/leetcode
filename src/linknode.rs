use std::cmp::{Ordering, PartialOrd};
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} => {:?}", self.val, self.next))
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

pub fn vec_to_link(l: Vec<i32>) -> Option<Box<ListNode>> {
    if l.len() == 0 { return None; }
    let mut head = ListNode::new(l[0]);
    let mut p = &mut head;
    for &i in &l[1..] {
        p.next = Some(Box::new(ListNode::new(i)));
        p = p.next.as_mut().unwrap();
    }
    Some(Box::new(head))
}

pub fn link_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut p = &head;
    while p.is_some() {
        result.push(p.as_ref().unwrap().val);
        p = &p.as_ref().unwrap().next;
    }
    result
}
