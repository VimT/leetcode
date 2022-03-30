//! 任务调度器

use std::collections::BinaryHeap;

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut m = [0; 26];
    for task in tasks {
        m[(task as u8 - b'A') as usize] += 1;
    }
    let mut heap = BinaryHeap::new();
    for i in 0..26 {
        if m[i] > 0 {
            heap.push((0, i));
            m[i] -= 1;
        }
    }
    let mut result = 0;
    while !heap.is_empty() {
        let (start_time, task) = heap.pop().unwrap();
        let start_time = -start_time;
        if result < start_time {
            result = start_time;
        }
        if m[task] > 0 {
            m[task] -= 1;
            heap.push((-start_time - n - 1, task));
        }
        result += 1;
    }
    result
}

fn main() {
    assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
    assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
    assert_eq!(least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'], 2), 16);
}
