//! 会议室 II

use std::collections::BinaryHeap;

/// 小顶堆存当前最早结束的会议时间
pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    intervals.sort_unstable();
    for interval in intervals {
        if !heap.is_empty() && -*heap.peek().unwrap() <= interval[0] {
            heap.pop().unwrap();
        }
        heap.push(-interval[1]);
    }
    heap.len() as i32
}

fn main() {
    fn test(func: fn(intervals: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 30], vec![5, 10], vec![15, 20]]), 2);
        assert_eq!(func(vec![vec![7, 10], vec![2, 4]]), 1);
    }
    test(min_meeting_rooms);
}
