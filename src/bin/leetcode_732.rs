//! 我的日程安排表 III


use std::collections::VecDeque;
use std::ptr::null_mut;

struct Node {
    left: *mut Node,
    right: *mut Node,
    s: u32,
    t: u32,
    lazy: i32,
    cnt: i32,
}

impl Node {
    fn new(s: u32, t: u32, cnt: i32) -> *mut Node {
        let node = Box::new(Node { left: null_mut(), right: null_mut(), s, t, lazy: 0, cnt });
        Box::into_raw(node)
    }
}

struct SegmentTree {
    root: *mut Node,
}

impl Drop for SegmentTree {
    fn drop(&mut self) {
        let mut q = VecDeque::new();
        q.push_back(self.root);
        unsafe {
            while !q.is_empty() {
                let node = q.pop_front().unwrap();
                if !(*node).left.is_null() {
                    q.push_back((*node).left);
                }
                if !(*node).right.is_null() {
                    q.push_back((*node).right);
                }
                Box::from_raw(node);
            }
        }
    }
}

const MIN: u32 = 0;
const MAX: u32 = 1e9 as u32 - 1;

impl SegmentTree {
    fn new() -> Self {
        Self { root: Node::new(MIN, MAX, 0) }
    }

    unsafe fn push_down(&mut self, p: *mut Node) {
        let target = (*p).lazy;
        if !(*p).left.is_null() {
            (*(*p).left).cnt += target;
            (*(*p).left).lazy += target;
        }
        if !(*p).right.is_null() {
            (*(*p).right).cnt += target;
            (*(*p).right).lazy += target;
        }
        (*p).lazy = 0;
    }

    unsafe fn update(&mut self, l: u32, r: u32, p: *mut Node) {
        if p.is_null() { return; }
        if (*p).lazy > 0 {
            self.push_down(p);
        }
        if (*p).s >= l && (*p).t <= r {
            (*p).cnt += 1;
            (*p).lazy += 1;
            return;
        }
        let mid = ((*p).s + (*p).t) / 2;
        if (*p).left.is_null() {
            (*p).left = Node::new((*p).s, mid, (*p).cnt);
        }
        if (*p).right.is_null() {
            (*p).right = Node::new(mid + 1, (*p).t, (*p).cnt);
        }
        if l <= mid {
            self.update(l, r, (*p).left);
        }
        if r > mid {
            self.update(l, r, (*p).right);
        }
        (*p).cnt = (*p).left.as_ref().map(|x| x.cnt).unwrap_or(0).max((*p).right.as_ref().map(|x| x.cnt).unwrap_or(0));
    }
}

struct MyCalendarThree {
    tree: SegmentTree,
}


impl MyCalendarThree {
    fn new() -> Self {
        Self { tree: SegmentTree::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        let start = start as u32;
        let end = end as u32 - 1;
        unsafe {
            self.tree.update(start, end, self.tree.root);
            (*self.tree.root).cnt as i32
        }
    }
}

fn main() {
    let mut c = MyCalendarThree::new();
    assert_eq!(c.book(10, 20), 1); // 返回 1 ，第一个日程安排可以预订并且不存在相交，所以最大 k 次预订是 1 次预订。
    assert_eq!(c.book(50, 60), 1); // 返回 1 ，第二个日程安排可以预订并且不存在相交，所以最大 k 次预订是 1 次预订。
    assert_eq!(c.book(10, 40), 2); // 返回 2 ，第三个日程安排 [10, 40) 与第一个日程安排相交，所以最大 k 次预订是 2 次预订。
    assert_eq!(c.book(5, 15), 3); // 返回 3 ，剩下的日程安排的最大 k 次预订是 3 次预订。
    assert_eq!(c.book(5, 10), 3); // 返回 3
    assert_eq!(c.book(25, 55), 3); // 返回 3
}