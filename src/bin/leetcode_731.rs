//! 我的日程安排表 II


use std::collections::VecDeque;
use std::ptr::null_mut;

struct Node {
    left: *mut Node,
    right: *mut Node,
    s: u32,
    t: u32,
    lazy: Option<bool>,
    cnt: u8,
}

impl Node {
    fn new(s: u32, t: u32, cnt: u8) -> *mut Node {
        let node = Box::new(Node { left: null_mut(), right: null_mut(), s, t, lazy: None, cnt });
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
                let _ = Box::from_raw(node);
            }
        }
    }
}

impl SegmentTree {
    fn new() -> Self {
        Self { root: Node::new(0, 1e9 as u32 - 1, 0) }
    }

    unsafe fn push_down(&mut self, p: *mut Node) {
        let target = (*p).lazy.unwrap();
        if !(*p).left.is_null() {
            (*(*p).left).cnt += 1;
            (*(*p).left).lazy = Some(target);
        }
        if !(*p).right.is_null() {
            (*(*p).right).cnt += 1;
            (*(*p).right).lazy = Some(target);
        }
        (*p).lazy = None;
    }

    unsafe fn query(&mut self, l: u32, r: u32, p: *mut Node) -> u8 {
        if (*p).s >= l && (*p).t <= r {
            return (*p).cnt;
        }
        let mid = ((*p).s + (*p).t) / 2;
        if (*p).lazy.is_some() {
            self.push_down(p);
        }
        let mut result = 0;
        if l <= mid {
            result = result.max(if (*p).left.is_null() { (*p).cnt } else { self.query(l, r, (*p).left) });
        }
        if r > mid {
            result = result.max(if (*p).right.is_null() { (*p).cnt } else { self.query(l, r, (*p).right) });
        }
        result
    }

    unsafe fn update(&mut self, l: u32, r: u32, p: *mut Node) {
        if p.is_null() { return; }
        if (*p).s >= l && (*p).t <= r {
            (*p).cnt += 1;
            (*p).lazy = Some(true);
            return;
        }
        let mid = ((*p).s + (*p).t) / 2;
        if (*p).lazy.is_some() {
            self.push_down(p);
        }
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
        (*p).cnt = (*p).left.as_ref().map(|x| x.cnt).unwrap_or(0).max((*p).right.as_ref().map(|x| x.cnt).unwrap_or(0))
    }
}

struct MyCalendarTwo {
    tree: SegmentTree,
}


impl MyCalendarTwo {
    fn new() -> Self {
        Self { tree: SegmentTree::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        unsafe {
            let end = end as u32 - 1;
            if self.tree.query(start as u32, end, self.tree.root) == 2 {
                return false;
            }
            self.tree.update(start as u32, end, self.tree.root);
            true
        }
    }
}

mod diff {
    use std::collections::BTreeMap;

    struct MyCalendarTwo {
        diff: BTreeMap<i32, i32>,
    }

    impl MyCalendarTwo {
        fn new() -> Self {
            Self { diff: BTreeMap::new() }
        }

        fn book(&mut self, start: i32, end: i32) -> bool {
            *self.diff.entry(start).or_insert(0) += 1;
            *self.diff.entry(end).or_insert(0) -= 1;

            let mut active = 0;
            for (_, &v) in self.diff.iter() {
                active += v;
                if active >= 3 {
                    *self.diff.entry(start).or_insert(0) -= 1;
                    *self.diff.entry(end).or_insert(0) += 1;
                    return false;
                }
            }
            true
        }
    }

    pub fn test() {
        let mut c = MyCalendarTwo::new();
        assert_eq!(c.book(10, 20), true);
        assert_eq!(c.book(50, 60), true);
        assert_eq!(c.book(10, 40), true);
        assert_eq!(c.book(5, 15), false);
        assert_eq!(c.book(5, 10), true);
        assert_eq!(c.book(25, 55), true);
    }
}

fn main() {
    let mut c = MyCalendarTwo::new();
    assert_eq!(c.book(10, 20), true);
    assert_eq!(c.book(50, 60), true);
    assert_eq!(c.book(10, 40), true);
    assert_eq!(c.book(5, 15), false);
    assert_eq!(c.book(5, 10), true);
    assert_eq!(c.book(25, 55), true);

    diff::test();
}