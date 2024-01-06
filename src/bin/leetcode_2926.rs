//! 平衡子序列的最大和

use leetcode::bit_index_tree::{BITree, BitVal};
use leetcode::segment_tree::{SegmentTree, SegmentSpec};


/// b[i] = nums[i] - i，问题变成从 b 选一个非降子序列，求对应的 nums 的元素和的最大值。
/// dp[i] 表示以 nums[i] 结尾的子序列的最大和。dp[i] = max(dp[j]) + nums[i]，其中 j < i 且 b[j] <= b[i]。
/// 用线段树维护 dp
pub fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
    pub enum Max {}
    impl SegmentSpec for Max {
        type ValType = i64;
        type LazyType = i64;
        fn op(&a: &Self::ValType, &b: &Self::ValType) -> Self::ValType { a.max(b) }
        fn identity() -> Self::ValType { i64::MIN }
        fn compose(&f: &Self::LazyType, g: &Self::LazyType) -> Self::LazyType { f.max(*g) }
        fn apply(&f: &Self::LazyType, a: &Self::ValType, _: i64) -> Self::ValType { f.max(*a) }
    }
    let mut b: Vec<i32> = nums.iter().copied().zip(0..).map(|(a, b)| a - b).collect();
    b.sort_unstable();
    b.dedup();
    let mut tree = SegmentTree::<Max>::new_with_size(b.len());
    let len = nums.len();
    for i in 0..len {
        let j = b.binary_search(&(nums[i] - i as i32)).unwrap(); // nums[i]-i 离散化的值
        let x = tree.query(0, j).max(0) + nums[i] as i64;
        tree.update(j, j, &x);
    }
    tree.query(0, b.len())
}

pub enum Max {}

impl BitVal for Max {
    type ValType = i64;
    fn identity() -> Self::ValType { i64::MIN }
    fn op(&f: &Self::ValType, &g: &Self::ValType) -> Self::ValType { return f.max(g); }
}

/// 树状数组
pub fn max_balanced_subsequence_sum2(nums: Vec<i32>) -> i64 {
    let mut b: Vec<i32> = nums.iter().copied().zip(0..).map(|(a, b)| a - b).collect();
    b.sort_unstable();
    b.dedup();
    let mut tree = BITree::<Max>::new(b.len());
    let len = nums.len();
    for i in 0..len {
        let j = b.binary_search(&(nums[i] - i as i32)).unwrap(); // nums[i]-i 离散化的值
        let x = tree.query_prefix(j + 1).max(0) + nums[i] as i64;
        tree.update(j, &x);
    }
    tree.query_prefix(b.len())
}

/// 不用二分查找找索引，时间更快
pub fn max_balanced_subsequence_sum3(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut dp = BITree::<Max>::new(n);
    let mut val = nums.iter().zip(0..).map(|(&x, i)| (x - i, i)).collect::<Vec<_>>();
    val.sort_unstable();
    let mut rank = vec![0; n];
    for (i, (_, ind)) in val.into_iter().enumerate() {
        rank[ind as usize] = i;
    }
    for (i, r) in rank.into_iter().enumerate() {
        dp.update(r, &(dp.query_prefix(r).max(0) + nums[i] as i64));
    }
    dp.query_prefix(n)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![34, 34, 32, 33]), 65);
        assert_eq!(func(vec![6, 4, 8, 9, -1]), 23);
        assert_eq!(func(vec![4, -1, 5, -1, -7]), 5);
        assert_eq!(func(vec![3, 3, 5, 6]), 14);
        assert_eq!(func(vec![5, -1, -3, 8]), 13);
        assert_eq!(func(vec![-2, -1]), -1);
    }
    test(max_balanced_subsequence_sum);
    test(max_balanced_subsequence_sum2);
    test(max_balanced_subsequence_sum3);
}
