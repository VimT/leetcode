//! 最大子序列的分数

use std::collections::BinaryHeap;

pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut num: Vec<(i32, i32)> = nums2.into_iter().zip(nums1).collect();
    num.sort_unstable();
    let len = num.len();
    let mut result = 0;
    let mut sum = 0;
    let k = k as usize;
    let mut q = BinaryHeap::with_capacity(k + 1);
    for i in (0..len).rev() {
        let (multi, a) = num[i];
        sum += a as i64;
        q.push(-a);
        if q.len() > k {
            sum += q.pop().unwrap() as i64;
        }
        if q.len() == k {
            result = result.max(sum * multi as i64);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![2,1,14,12], vec![11,7,13,6], 3), 168);
        assert_eq!(func(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3), 12);
        assert_eq!(func(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1), 30);
    }
    test(max_score);
}
