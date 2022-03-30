//! 吃苹果的最大数目

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    let len = apples.len();
    let mut heap = BinaryHeap::new();
    let mut result = 0;
    let mut day = 0;
    heap.push(Reverse((days[0] as usize - 1, apples[0])));
    while !heap.is_empty() {
        while !heap.is_empty() && (day > heap.peek().unwrap().0.0 || heap.peek().unwrap().0.1 == 0) {
            heap.pop().unwrap();
        }
        if !heap.is_empty() {
            heap.peek_mut().unwrap().0.1 -= 1;
            result += 1;
        }
        day += 1;
        if day < len {
            heap.push(Reverse((day + days[day] as usize - 1, apples[day])));
        }
    }
    result
}

fn main() {
    assert_eq!(eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]), 7);
    assert_eq!(eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2]), 5);
}
