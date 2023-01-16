//! 过桥的时间

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug, Ord)]
struct Worker {
    i: usize,
    l2r: i32,
    r2l: i32,
    pick_old: i32,
    put_new: i32,
}


impl PartialOrd<Self> for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.l2r + self.r2l, self.i).partial_cmp(&(other.l2r + other.r2l, other.i))
    }
}

pub fn find_crossing_time(mut n: i32, _: i32, time: Vec<Vec<i32>>) -> i32 {
    let workers: Vec<Worker> = time.into_iter().enumerate().map(|(i, x)| Worker { i, l2r: x[0], r2l: x[2], pick_old: x[1], put_new: x[3] }).collect();
    let mut left_wait: BinaryHeap<Worker> = BinaryHeap::new();
    let mut right_wait: BinaryHeap<Worker> = BinaryHeap::new();
    let mut wait2_right: BinaryHeap<(i32, Worker)> = BinaryHeap::new();
    let mut wait2_left: BinaryHeap<(i32, Worker)> = BinaryHeap::new();
    let mut cur_time = 0;
    for worker in workers {
        left_wait.push(worker);
    }
    while n > 0 {
        while !wait2_left.is_empty() && -wait2_left.peek().unwrap().0 <= cur_time {
            left_wait.push(wait2_left.pop().unwrap().1); // 左边放完箱子，到左桥
        }
        while !wait2_right.is_empty() && -wait2_right.peek().unwrap().0 <= cur_time {
            right_wait.push(wait2_right.pop().unwrap().1); // 右边放完箱子，到右桥
        }
        if !right_wait.is_empty() { // 右边过桥
            let w = right_wait.pop().unwrap();
            cur_time += w.r2l;
            wait2_left.push((-cur_time - w.put_new, w));
        } else if !left_wait.is_empty() { // 左边过桥
            let w = left_wait.pop().unwrap();
            cur_time += w.l2r;
            wait2_right.push((-cur_time - w.pick_old, w));
            n -= 1;
        } else if wait2_left.is_empty() {
            cur_time = -wait2_right.peek().unwrap().0;
        } else if wait2_right.is_empty() {
            cur_time = -wait2_left.peek().unwrap().0;
        } else {
            cur_time = (-wait2_right.peek().unwrap().0).min(-wait2_left.peek().unwrap().0)
        }
    }
    while !wait2_right.is_empty() {
        let (t, w) = wait2_right.pop().unwrap();
        cur_time = cur_time.max(-t) + w.r2l;
    }
    cur_time
}

fn main() {
    fn test(func: fn(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(10, 6, vec![vec![2, 10, 5, 8], vec![3, 5, 2, 2], vec![5, 8, 10, 10], vec![7, 8, 8, 5], vec![5, 6, 6, 10], vec![6, 10, 6, 2]]), 149);
        assert_eq!(func(1, 3, vec![vec![1, 1, 2, 1], vec![1, 1, 3, 1], vec![1, 1, 4, 1]]), 6);
        assert_eq!(func(3, 2, vec![vec![1, 9, 1, 8], vec![10, 10, 10, 10]]), 50);
    }
    test(find_crossing_time);
}
