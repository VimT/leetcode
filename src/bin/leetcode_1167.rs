//! 连接棒材的最低费用

use std::collections::BinaryHeap;

pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = sticks.into_iter().map(|x| -x).collect();
    let mut result = 0;
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        result += a + b;
        heap.push(a + b);
    }
    -result
}

fn main() {
    fn test(func: fn(sticks: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 4, 3]), 14);
        assert_eq!(func(vec![1, 8, 3, 5]), 30);
        assert_eq!(func(vec![5]), 0);
    }
    test(connect_sticks);
}
