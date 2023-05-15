//! 合并两个二维数组

use std::collections::BTreeMap;

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = BTreeMap::new();
    for num in nums1.into_iter().chain(nums2) {
        *result.entry(num[0]).or_default() += num[1];
    }
    result.into_iter().map(|x| vec![x.0, x.1]).collect()
}

fn main() {
    fn test(func: fn(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1, 2], vec![2, 3], vec![4, 5]], vec![vec![1, 4], vec![3, 2], vec![4, 1]]), vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]);
        assert_eq!(func(vec![vec![2, 4], vec![3, 6], vec![5, 5]], vec![vec![1, 3], vec![4, 3]]), vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]);
    }
    test(merge_arrays);
}
