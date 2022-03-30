//! Range 模块

use std::collections::VecDeque;
use std::ptr::null_mut;

struct Node {
    left: *mut Node,
    right: *mut Node,
    s: u32,
    t: u32,
    lazy: Option<bool>,
    ok: bool,
}

impl Node {
    fn new(s: u32, t: u32, ok: bool) -> *mut Node {
        let node = Box::new(Node { left: null_mut(), right: null_mut(), s, t, lazy: None, ok });
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

impl SegmentTree {
    fn new() -> Self {
        Self { root: Node::new(1, 1e9 as u32 - 1, false) }
    }

    unsafe fn push_down(&mut self, p: *mut Node) {
        let target = (*p).lazy.unwrap();
        if !(*p).left.is_null() {
            (*(*p).left).ok = target;
            (*(*p).left).lazy = Some(target);
        }
        if !(*p).right.is_null() {
            (*(*p).right).ok = target;
            (*(*p).right).lazy = Some(target);
        }
        (*p).lazy = None;
    }

    unsafe fn query(&mut self, l: u32, r: u32, p: *mut Node) -> bool {
        if (*p).s >= l && (*p).t <= r {
            return (*p).ok;
        }
        let mid = ((*p).s + (*p).t) / 2;
        if (*p).lazy.is_some() {
            self.push_down(p);
        }
        let mut result = true;
        if l <= mid {
            result &= if (*p).left.is_null() { (*p).ok } else { self.query(l, r, (*p).left) };
        }
        if r > mid && result {
            result &= if (*p).right.is_null() { (*p).ok } else { self.query(l, r, (*p).right) }
        }
        result
    }

    unsafe fn update(&mut self, l: u32, r: u32, p: *mut Node, target: bool) {
        if p.is_null() { return; }
        if (*p).s >= l && (*p).t <= r {
            (*p).ok = target;
            (*p).lazy = Some(target);
            return;
        }
        let mid = ((*p).s + (*p).t) / 2;
        if (*p).lazy.is_some() {
            self.push_down(p);
        }
        if (*p).left.is_null() {
            (*p).left = Node::new((*p).s, mid, (*p).ok);
        }
        if (*p).right.is_null() {
            (*p).right = Node::new(mid + 1, (*p).t, (*p).ok);
        }
        if l <= mid {
            self.update(l, r, (*p).left, target);
        }
        if r > mid {
            self.update(l, r, (*p).right, target);
        }
        (*p).ok = (*p).left.as_ref().map(|x| x.ok).unwrap_or(false) & (*p).right.as_ref().map(|x| x.ok).unwrap_or(false)
    }
}

struct RangeModule {
    tree: SegmentTree,
}


impl RangeModule {
    fn new() -> Self {
        Self { tree: SegmentTree::new() }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        unsafe {
            self.tree.update(left as u32, right as u32 - 1, self.tree.root, true);
        }
    }

    fn query_range(&mut self, left: i32, right: i32) -> bool {
        unsafe {
            self.tree.query(left as u32, right as u32 - 1, self.tree.root)
        }
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        unsafe {
            self.tree.update(left as u32, right as u32 - 1, self.tree.root, false);
        }
    }
}

mod stdcode {
    use std::cmp::Ordering::{Equal, Greater, Less};

    struct RangeModule {
        itvs: Vec<(i32, i32)>,
    }

    impl RangeModule {
        fn new() -> Self {
            Self { itvs: Vec::new() }
        }

        fn add_range(&mut self, mut left: i32, mut right: i32) {
            let mut next_itvs = Vec::with_capacity(self.itvs.len() + 1);
            let mut idx = 0;
            while idx < self.itvs.len() && self.itvs[idx].1 < left {
                next_itvs.push(self.itvs[idx]);
                idx += 1;
            }

            while idx < self.itvs.len() && self.itvs[idx].0 <= right {
                left = self.itvs[idx].0.min(left);
                right = self.itvs[idx].1.max(right);
                idx += 1;
            }

            next_itvs.push((left, right));
            next_itvs.append(&mut self.itvs[idx..].to_vec());
            self.itvs = next_itvs;
            // println!("{:?}", self.itvs);
        }

        fn query_range(&self, left: i32, right: i32) -> bool {
            // println!("{:?}", self.itvs);
            let mut idx = 0;
            while idx < self.itvs.len() && self.itvs[idx].1 < left {
                idx += 1;
            }

            idx < self.itvs.len() && self.itvs[idx].0 <= left && self.itvs[idx].1 >= right
        }

        fn remove_range(&mut self, left: i32, right: i32) {
            let mut next_itvs = Vec::with_capacity(self.itvs.len() + 1);

            for &(curr_left, curr_right) in self.itvs.iter() {
                if curr_right < left {
                    next_itvs.push((curr_left, curr_right));
                } else if curr_left >= right {
                    next_itvs.push((curr_left, curr_right));
                } else {
                    match (curr_left.cmp(&left), curr_right.cmp(&right)) {
                        (Less, Greater) => {
                            next_itvs.push((curr_left, left));
                            next_itvs.push((right, curr_right));
                        }
                        (Less, _) => next_itvs.push((curr_left, left)),
                        (_, Greater) => next_itvs.push((right, curr_right)),
                        _ => (),
                    }
                }
            }

            self.itvs = next_itvs;
            // println!("{:?}", self.itvs);
        }
    }
}

fn main() {
    let mut range = RangeModule::new();
    range.add_range(10, 20);
    range.remove_range(14, 16);
    assert_eq!(range.query_range(10, 14), true);
    assert_eq!(range.query_range(13, 15), false);
    assert_eq!(range.query_range(16, 17), true);

    range = RangeModule::new();
    range.add_range(6, 8);
    range.remove_range(7, 8);
    range.remove_range(8, 9);
    range.add_range(8, 9);
    range.remove_range(1, 3);
    range.add_range(1, 8);
    assert_eq!(range.query_range(2, 4), true);
    assert_eq!(range.query_range(2, 9), true);
    assert_eq!(range.query_range(4, 6), true);
}
