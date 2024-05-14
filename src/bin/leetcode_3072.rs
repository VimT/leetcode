//! 将元素分配到两个数组中 II

use std::cmp::Ordering;

use leetcode::segment_tree::{DynamicSegmentTree, SegmentSpec};

pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![];
    let mut arr2 = vec![];
    arr1.push(nums[0]);
    arr2.push(nums[1]);
    let len = nums.len();
    const MX: i64 = 1e9 as i64 + 1;
    let mut tree1 = DynamicSegmentTree::new(len, 0, MX);
    let mut tree2 = DynamicSegmentTree::new(len, 0, MX);
    tree1.update(nums[0] as i64, nums[0] as i64, &1);
    tree2.update(nums[1] as i64, nums[1] as i64, &1);

    pub enum Sum {}

    impl SegmentSpec for Sum {
        type ValType = i64;
        type LazyType = i64;
        fn op(&a: &Self::ValType, &b: &Self::ValType) -> Self::ValType { a + b }
        fn identity() -> Self::ValType { 0 }
        fn compose(&f: &Self::LazyType, g: &Self::LazyType) -> Self::LazyType { f + g }
        fn apply(&f: &Self::LazyType, a: &Self::ValType, size: i64) -> Self::ValType { a + f * size }
    }

    fn greater_count(tree: &mut DynamicSegmentTree<Sum>, x: i32) -> usize {
        return tree.query(x as i64 + 1, MX) as usize;
    }

    for i in 2..len {
        let c1 = greater_count(&mut tree1, nums[i]);
        let c2 = greater_count(&mut tree2, nums[i]);
        match c1.cmp(&c2) {
            Ordering::Less => {
                arr2.push(nums[i]);
                tree2.update(nums[i] as i64, nums[i] as i64, &1);
            }
            Ordering::Equal => {
                if arr1.len() <= arr2.len() {
                    arr1.push(nums[i]);
                    tree1.update(nums[i] as i64, nums[i] as i64, &1);
                } else {
                    arr2.push(nums[i]);
                    tree2.update(nums[i] as i64, nums[i] as i64, &1);
                }
            }
            Ordering::Greater => {
                arr1.push(nums[i]);
                tree1.update(nums[i] as i64, nums[i] as i64, &1);
            }
        }
    }
    arr1.extend_from_slice(&arr2);
    arr1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![2, 1, 3, 3]), vec![2, 3, 1, 3]);
        assert_eq!(func(vec![5, 14, 3, 1, 2]), vec![5, 3, 1, 2, 14]);
        assert_eq!(func(vec![3, 3, 3, 3]), vec![3, 3, 3, 3]);
    }
    test(result_array);
}
