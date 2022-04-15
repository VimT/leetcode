//! 餐盘栈

use std::cmp::Reverse;
use std::collections::BinaryHeap;
// -------------------------------
use std::ptr::null_mut;

#[derive(Default)]
struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    left_not_full: BinaryHeap<Reverse<usize>>,
    right_not_empty: BinaryHeap<usize>,
}

impl DinnerPlates {
    pub fn new(capacity: i32) -> Self {
        let mut dp: Self = Default::default();
        dp.capacity = capacity as usize;
        dp
    }

    fn push(&mut self, val: i32) {
        if self.left_not_full.is_empty() {
            let len = self.stacks.len();
            self.left_not_full.push(Reverse(len));
            self.right_not_empty.push(len);
            self.stacks.push(Vec::with_capacity(self.capacity));
        }
        let Reverse(idx) = self.left_not_full.pop().unwrap();
        if self.stacks[idx].is_empty() {
            self.right_not_empty.push(idx);
        }
        self.stacks[idx].push(val);
        if self.stacks[idx].len() != self.capacity {
            self.left_not_full.push(Reverse(idx));
        }
    }

    fn pop(&mut self) -> i32 {
        // 在pop_at_stack的时候，可能把某个栈变成空但是却没有更新right_not_empty，所以这里判断一下
        while !self.right_not_empty.is_empty() && self.stacks[*self.right_not_empty.peek().unwrap()].is_empty() {
            self.right_not_empty.pop().unwrap();
        }
        if let Some(idx) = self.right_not_empty.pop() {
            if self.stacks[idx].len() == self.capacity {
                self.left_not_full.push(Reverse(idx));
            }
            let result = self.stacks[idx].pop().unwrap();
            if !self.stacks[idx].is_empty() {
                self.right_not_empty.push(idx);
            }
            result
        } else {
            -1
        }
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let idx = index as usize;
        if let Some(s) = self.stacks.get_mut(idx) {
            if s.len() == self.capacity {
                self.left_not_full.push(Reverse(idx));
            }
            return s.pop().unwrap_or(-1);
        }
        -1
    }
}

struct Node {
    left: *mut Node,
    right: *mut Node,
    s: u32,
    t: u32,
    cnt: i32,
}

impl Node {
    fn new(s: u32, t: u32, cnt: i32) -> *mut Node {
        let node = Box::new(Node { left: null_mut(), right: null_mut(), s, t, cnt });
        Box::into_raw(node)
    }
}

const MAX: u32 = 200005;

/// 线段树
struct DinnerPlates2 {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    tree: *mut Node,
}

impl DinnerPlates2 {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: vec![],
            tree: Node::new(0, MAX, 0),
        }
    }

    fn push(&mut self, val: i32) {
        let mut l = 0;
        let mut r = MAX;
        unsafe {
            while l < r {
                let m = (l + r) >> 1;
                if self.query(0, m, self.tree) as usize == (m + 1) as usize * self.capacity {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            self.update(l, self.tree, 1);
            if l as usize == self.stacks.len() {
                self.stacks.push(Vec::with_capacity(self.capacity));
            }
            self.stacks[l as usize].push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        let mut l = 1;
        let mut r = MAX;
        unsafe {
            while l < r {
                let mid = (l + r) >> 1;
                if self.query(mid, MAX, self.tree) > 0 {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            if self.stacks[l as usize - 1].is_empty() {
                return -1;
            }
            self.update(l - 1, self.tree, -1);
            self.stacks[l as usize - 1].pop().unwrap()
        }
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        unsafe {
            if (index as usize) >= self.stacks.len() || self.stacks[index as usize].is_empty() {
                return -1;
            }
            self.update(index as u32, self.tree, -1);
            self.stacks[index as usize].pop().unwrap()
        }
    }

    unsafe fn update(&mut self, idx: u32, p: *mut Node, diff: i32) {
        if (*p).s == (*p).t && (*p).s == idx {
            (*p).cnt += diff;
            return;
        }
        let mid = ((*p).s + (*p).t) / 2;
        if idx <= mid {
            if (*p).left.is_null() {
                (*p).left = Node::new((*p).s, mid, 0);
            }
            self.update(idx, (*p).left, diff);
        } else {
            if (*p).right.is_null() {
                (*p).right = Node::new(mid + 1, (*p).t, 0);
            }
            self.update(idx, (*p).right, diff);
        }
        (*p).cnt = (*p).left.as_ref().map(|x| x.cnt).unwrap_or(0) + (*p).right.as_ref().map(|x| x.cnt).unwrap_or(0)
    }

    unsafe fn query(&self, l: u32, r: u32, p: *mut Node) -> i32 {
        if (*p).s >= l && (*p).t <= r {
            return (*p).cnt;
        }
        let mid = ((*p).s + (*p).t) / 2;
        let mut result = 0;
        if l <= mid && !(*p).left.is_null() {
            result += self.query(l, r, (*p).left);
        }
        if r > mid && !(*p).right.is_null() {
            result += self.query(l, r, (*p).right);
        }
        result
    }
}


fn main() {
    let mut d = DinnerPlates::new(1);
    d.push(1);
    d.push(2);
    assert_eq!(d.pop_at_stack(1), 2);
    assert_eq!(d.pop(), 1);
    d.push(1);
    d.push(2);
    assert_eq!(d.pop(), 2);
    assert_eq!(d.pop(), 1);

    let mut d = DinnerPlates2::new(2);
    d.push(1);
    d.push(2);
    d.push(3);
    d.push(4);
    d.push(5);
    assert_eq!(d.pop_at_stack(0), 2);
    d.push(20);
    d.push(21);
    assert_eq!(d.pop_at_stack(0), 20);
    assert_eq!(d.pop_at_stack(2), 21);
    assert_eq!(d.pop(), 5);
    assert_eq!(d.pop(), 4);
    assert_eq!(d.pop(), 3);
    assert_eq!(d.pop(), 1);
    assert_eq!(d.pop(), -1);

    let mut d = DinnerPlates2::new(2);
    d.push(1);
    d.push(2);
    d.push(3);
    d.push(4);
    d.push(7);
    assert_eq!(d.pop_at_stack(8), -1);
    d.push(20);
    d.push(21);
    assert_eq!(d.pop_at_stack(0), 2);
    assert_eq!(d.pop_at_stack(2), 20);
    assert_eq!(d.pop(), 21);
    assert_eq!(d.pop(), 7);
    assert_eq!(d.pop(), 4);
    assert_eq!(d.pop(), 3);
    assert_eq!(d.pop(), 1);
}
