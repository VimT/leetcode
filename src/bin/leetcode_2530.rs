//! 执行 K 次操作后的最大分数

use std::collections::BinaryHeap;

pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
    let mut result = 0;
    for _ in 0..k {
        if heap.is_empty() { break; }
        let score = heap.pop().unwrap();
        result += score as i64;
        heap.push((score + 2) / 3); // ceil 的快算写法
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![10, 10, 10, 10, 10], 5), 50);
        assert_eq!(func(vec![1, 10, 3, 3, 3], 3), 17);
    }
    test(max_kelements);
}
