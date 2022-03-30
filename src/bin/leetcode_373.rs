//! 查找和最小的K对数字

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut heap = BinaryHeap::new();
    let len1 = nums1.len();
    let len2 = nums2.len();
    for i in 0..(k as usize).min(len1) {
        heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
    }
    let mut result = vec![];
    for _ in 0..k {
        if heap.is_empty() { break; }
        let Reverse((_, i, j)) = heap.pop().unwrap();
        result.push(vec![nums1[i], nums2[j]]);
        if j + 1 < len2 {
            heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
        }
    }
    result
}

fn main() {
    assert_eq!(k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3), vec![vec![1, 2], vec![1, 4], vec![1, 6]]);
    assert_eq!(k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2), vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(k_smallest_pairs(vec![1, 2], vec![3], 3), vec![vec![1, 3], vec![2, 3]]);
}

