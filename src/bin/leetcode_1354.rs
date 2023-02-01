//! 多次求和构造目标数组

use std::collections::BinaryHeap;

/// 倒推 + 考虑[1, 1e9]的情况
pub fn is_possible(target: Vec<i32>) -> bool {
    let len = target.len();
    let mut heap: BinaryHeap<i64> = BinaryHeap::with_capacity(len);
    let mut sum = 0;
    for num in target {
        sum += num as i64;
        if num > 1 {
            heap.push(num as i64);
        }
    }
    while let Some(num) = heap.pop() {
        let rest = sum - num;
        if rest >= num || rest < 1 { return false; }
        let next = num % rest;
        if next == 0 && rest != 1 {
            return false;
        }
        sum = rest + next;
        if next > 1 {
            heap.push(next);
        }
    }
    true
}

fn main() {
    fn test(func: fn(target: Vec<i32>) -> bool) {
        assert_eq!(func(vec![2]), false);
        assert_eq!(func(vec![2, 900000002]), false);
        assert_eq!(func(vec![1, 1000000000]), true);
        assert_eq!(func(vec![9, 9, 9]), false);
        assert_eq!(func(vec![9, 3, 5]), true);
        assert_eq!(func(vec![1, 1, 1, 2]), false);
        assert_eq!(func(vec![8, 5]), true);
    }
    test(is_possible);
}
