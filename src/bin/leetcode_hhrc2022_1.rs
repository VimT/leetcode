//! 化学反应

use std::collections::BinaryHeap;

pub fn last_material(material: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = material.into_iter().collect();
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        if a != b{
            heap.push(a - b);
        }
    }
    heap.pop().unwrap_or(0)
}

fn main() {
    assert_eq!(last_material(vec![1,3]), 2);
    assert_eq!(last_material(vec![10, 2, 6, 1]), 1);
    assert_eq!(last_material(vec![6, 4, 10]), 0);
}
