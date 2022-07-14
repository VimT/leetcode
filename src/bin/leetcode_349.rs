//! 两个数组的交集

use std::collections::HashSet;
use leetcode::unorder;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    nums1.into_iter().collect::<HashSet<i32>>().intersection(&nums2.into_iter().collect::<HashSet<i32>>()).cloned().collect()
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>) {
        assert_eq!(unorder(func(vec![1, 2, 2, 1], vec![2, 2])), vec![2]);
        assert_eq!(unorder(func(vec![4, 9, 5], vec![9, 4, 9, 8, 4])), vec![4, 9]);
    }
    test(intersection);
}
