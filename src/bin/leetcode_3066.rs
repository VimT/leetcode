//! 超过阈值的最少操作数 II

use std::collections::BinaryHeap;

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<i64> = nums.into_iter().map(|x| -x as i64).collect();
    let mut cnt = 0;
    let k = k as i64;
    while heap.len() >= 2 && -heap.peek().unwrap() < k {
        let a = -heap.pop().unwrap();
        let b = -heap.pop().unwrap();
        heap.push(-(a * 2 + b));
        cnt += 1;
    }
    cnt
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, 11, 10, 1, 3], 10), 2);
        assert_eq!(func(vec![1, 1, 2, 4, 9], 20), 4);
    }
    test(min_operations);
}
