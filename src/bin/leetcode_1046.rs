//! 最后一块石头的重量

use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    for stone in stones {
        heap.push(stone);
    }
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        if a > b {
            heap.push(a - b);
        }
    }
    heap.pop().unwrap_or(0)
}

fn main() {
    assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(last_stone_weight(vec![1]), 1);
    assert_eq!(last_stone_weight(vec![1, 1]), 0);
}
