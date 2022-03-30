//! 数组的最小偏移量


use std::collections::BinaryHeap;

pub fn minimum_deviation(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut q = BinaryHeap::new();
    let mut min = i32::MAX;
    for num in nums {
        let num = if num & 1 == 1 { num << 1 } else { num };
        min = min.min(num);
        q.push(num);
    }
    let mut result = *q.peek().unwrap() - min;
    while !q.is_empty() && *q.peek().unwrap() & 1 == 0 {
        let num = q.pop().unwrap() >> 1;
        q.push(num);
        min = min.min(num);
        result = result.min(*q.peek().unwrap() - min);
    }
    result
}


fn main() {
    assert_eq!(minimum_deviation(vec![1, 2, 3, 4]), 1);
    assert_eq!(minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
    assert_eq!(minimum_deviation(vec![2, 10, 8]), 3);
}
