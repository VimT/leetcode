//! 课程表 III

use std::collections::BinaryHeap;

/// 很巧妙的解法
pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
    let mut queue = BinaryHeap::new();
    courses.sort_unstable_by_key(|x| x[1]);
    let mut time = 0;
    for c in courses {
        if time + c[0] <= c[1] {
            queue.push(c[0]);
            time += c[0];
        } else if !queue.is_empty() && *queue.peek().unwrap() > c[0] {
            time += c[0] - queue.pop().unwrap();
            queue.push(c[0]);
        }
    }
    queue.len() as i32
}

fn main() {
    assert_eq!(schedule_course(vec![vec![100, 200], vec![200, 1300], vec![1000, 1250], vec![2000, 3200]]), 3);
    assert_eq!(schedule_course(vec![vec![1, 2]]), 1);
    assert_eq!(schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
}
