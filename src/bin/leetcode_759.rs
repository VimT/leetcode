//! 员工空闲时间

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    fn new(start: i32, end: i32) -> Self {
        Interval {
            start,
            end,
        }
    }
}

/// 扫描线
pub fn employee_free_time(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    enum S {
        Open = 0,
        Close = 1,
    }
    let mut events = vec![];
    for emp in schedule {
        for iv in emp {
            events.push((iv.start, S::Open));
            events.push((iv.end, S::Close));
        }
    }
    events.sort_unstable();
    let mut result = vec![];
    let mut prev = None;
    let mut bal = 0;
    for (t, cmd) in events {
        if bal == 0 && prev.is_some() {
            result.push(Interval::new(prev.unwrap(), t));
        }
        bal += match cmd {
            S::Open => 1,
            S::Close => -1,
        };
        prev = Some(t);
    }
    result
}

/// 优先队列
pub fn employee_free_time2(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
    let mut result = vec![];
    let mut heap = BinaryHeap::new();
    for (i, emp) in schedule.iter().enumerate() {
        heap.push((-emp[0].start, i, 0));
    }
    let mut anchor = schedule.iter().flat_map(|x| x.iter()).map(|x| x.start).min().unwrap();
    while !heap.is_empty() {
        let (t, e_id, e_jx) = heap.pop().unwrap();
        if anchor < -t {
            result.push(Interval::new(anchor, -t));
        }
        anchor = anchor.max(schedule[e_id][e_jx].end);
        if e_jx + 1 < schedule[e_id].len() {
            heap.push((-schedule[e_id][e_jx + 1].start, e_id, e_jx + 1));
        }
    }
    result
}

fn main() {
    fn test(func: fn(schedule: Vec<Vec<Interval>>) -> Vec<Interval>) {
        assert_eq!(func(vec![vec![Interval::new(1, 2), Interval::new(5, 6)], vec![Interval::new(1, 3)], vec![Interval::new(4, 10)]]), vec![Interval::new(3, 4)]);
        assert_eq!(func(vec![vec![Interval::new(1, 3), Interval::new(6, 7)], vec![Interval::new(2, 4)], vec![Interval::new(2, 5), Interval::new(9, 12)]]), vec![Interval::new(5, 6), Interval::new(7, 9)]);
    }
    test(employee_free_time);
    test(employee_free_time2);
}
