//! 删除元素后和的最小差值

use std::collections::BinaryHeap;

pub fn minimum_difference(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut cur_sum = 0;
    let mut left = vec![0; len];
    let mut right = vec![0; len];
    let mut heap = BinaryHeap::new();
    for i in 0..len / 3 * 2 {
        cur_sum += nums[i] as i64;
        heap.push(nums[i]);
        if heap.len() > len / 3 {
            cur_sum -= heap.pop().unwrap() as i64;
        }
        left[i] = cur_sum;
    }
    cur_sum = 0;
    heap.clear();
    for i in (len / 3..len).rev() {
        cur_sum += nums[i] as i64;
        heap.push(-nums[i]);
        if heap.len() > len / 3 {
            cur_sum += heap.pop().unwrap() as i64;
        }
        right[i] = cur_sum;
    }
    let mut result = i64::MAX;
    for i in len / 3 - 1..len / 3 * 2 {
        result = result.min(left[i] - right[i + 1]);
    }
    result
}

fn main() {
    assert_eq!(minimum_difference(vec![1, 2, 3]), -2);
    assert_eq!(minimum_difference(vec![1, 3, 2]), -2);
    assert_eq!(minimum_difference(vec![3, 1, 2]), -1);
    assert_eq!(minimum_difference(vec![7, 9, 5, 8, 1, 3]), 1);
}
