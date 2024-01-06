//! 子数组不同元素数目的平方和 II

use std::collections::HashMap;

use leetcode::segment_tree::{SegmentTree, SegmentSpec};

pub enum Sum {}

impl SegmentSpec for Sum {
    type ValType = i64;
    type LazyType = i64;
    fn op(&a: &Self::ValType, &b: &Self::ValType) -> Self::ValType { a + b }
    fn identity() -> Self::ValType { 0 }
    fn compose(&f: &Self::LazyType, g: &Self::LazyType) -> Self::LazyType { f + g }
    fn apply(&f: &Self::LazyType, a: &Self::ValType, size: i64) -> Self::ValType { a + f * size }
}

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1000000007;
    let mut seg = SegmentTree::<Sum>::new(&vec![0; nums.len() + 1]);
    let mut cnt: HashMap<i32, usize> = HashMap::new();
    let mut result = 0;
    let mut cur = 0;  // 前i个数的不同元素数目的平方和
    for (num, i) in nums.into_iter().zip(1..) {
        let last = cnt.get(&num).copied().unwrap_or(0);
        let more = 2 * seg.query(last + 1, i) + i as i64 - last as i64;  // (x+1)^2 - x^2 = 2x + 1
        cur = (cur + more) % MOD;
        result = (result + cur) % MOD;
        seg.update(last + 1, i, &1);
        cnt.insert(num, i);
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![5, 2, 4, 2, 1, 3, 2, 4, 3, 1]), 578);
        assert_eq!(func(vec![1, 2, 1]), 15);
        assert_eq!(func(vec![2, 2]), 3);
    }
    test(sum_counts);
}
