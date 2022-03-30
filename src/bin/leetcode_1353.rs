//! 最多可以参加的会议数目


use std::collections::BinaryHeap;

/// 贪心，按开始时间排序，遍历每一天，在这一天，优先做结束时间最小的
/// 使用优先队列实现
pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
    events.sort_unstable();
    let mut heap = BinaryHeap::new();
    let mut cur_day = 1;
    let mut i = 0;
    let len = events.len();
    let mut result = 0;
    while i < len || !heap.is_empty() {
        while i < len && events[i][0] == cur_day {
            heap.push(-events[i][1]);
            i += 1;
        }
        while !heap.is_empty() && -*heap.peek().unwrap() < cur_day {
            heap.pop().unwrap();
        }
        if !heap.is_empty() {
            heap.pop().unwrap();
            result += 1;
        }
        cur_day += 1;
    }
    result
}

fn main() {
    assert_eq!(max_events(vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![3, 4]]), 4);
    assert_eq!(max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]), 4);
    assert_eq!(max_events(vec![vec![1, 5], vec![1, 5], vec![1, 5], vec![2, 3], vec![2, 3]]), 5);
    assert_eq!(max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]), 3);
    assert_eq!(max_events(vec![vec![1, 4], vec![4, 4], vec![2, 2], vec![3, 4], vec![1, 1]]), 4);
    assert_eq!(max_events(vec![vec![1, 100000]]), 1);
    assert_eq!(max_events(vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7]]), 7);
}
