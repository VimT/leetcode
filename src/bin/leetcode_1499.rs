//! 满足不等式的最大值

use std::collections::{BinaryHeap, VecDeque};

pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut result = i32::MIN;
    for point in points {
        let (x, y) = (point[0], point[1]);
        while !heap.is_empty() && x - heap.peek().unwrap().1 > k {
            heap.pop();
        }
        if !heap.is_empty() {
            result = result.max(x + y + heap.peek().unwrap().0);
        }
        heap.push((y - x, x));
    }
    result
}

/// 单调减队列，可以O(n) （实际执行时间没有区别）
pub fn find_max_value_of_equation2(points: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut result = i32::MIN;
    for point in points {
        let (x, y) = (point[0], point[1]);
        while !q.is_empty() && x - q.front().unwrap().1 > k {
            q.pop_front();
        }
        if let Some((p, _)) = q.front() {
            result = result.max(x + y + *p);
        }
        let diff = y - x;
        while !q.is_empty() && diff >= q.back().unwrap().0 {
            q.pop_back();
        }
        q.push_back((diff, x));
    }
    result
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![1, 3], vec![2, 0], vec![5, 10], vec![6, -10]], 1), 4);
        assert_eq!(func(vec![vec![0, 0], vec![3, 0], vec![9, 2]], 3), 3);
    }
    test(find_max_value_of_equation);
    test(find_max_value_of_equation2);
}
