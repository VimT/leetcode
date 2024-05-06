//! 最高频率的 ID

use std::collections::{BinaryHeap, BTreeMap, HashSet};
use std::collections::btree_map::Entry;

use leetcode::segment_tree::{DynamicSegmentTree, SegmentSpec};

pub fn most_frequent_i_ds(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
    const MX: usize = 1e5 as usize + 1;
    let mut m: BTreeMap<i64, HashSet<i32>> = BTreeMap::new();
    let mut num_freq = vec![0; MX];
    nums.into_iter().zip(freq).map(|(num, freq)| {
        let before_freq = num_freq[num as usize];
        if before_freq > 0 {
            if let Entry::Occupied(mut entry) = m.entry(before_freq) {
                entry.get_mut().remove(&num);
                if entry.get().is_empty() { entry.remove(); }
            }
        }
        num_freq[num as usize] += freq as i64;
        m.entry(num_freq[num as usize]).or_default().insert(num);
        m.last_key_value().map(|(&k, _)| k).unwrap()
    }).collect()
}

pub fn most_frequent_i_ds2(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
    const MX: usize = 1e5 as usize + 1;
    pub enum AddMax {}

    // 更新是加，查询是求最大值
    impl SegmentSpec for AddMax {
        type ValType = i64;
        type LazyType = i64;
        fn op(&a: &Self::ValType, &b: &Self::ValType) -> Self::ValType { a.max(b) }
        fn identity() -> Self::ValType { 0 }
        fn compose(&f: &Self::LazyType, g: &Self::LazyType) -> Self::LazyType { f + g }
        fn apply(&f: &Self::LazyType, a: &Self::ValType, _: i64) -> Self::ValType { f + a }
    }

    let mut tree = DynamicSegmentTree::<AddMax>::new(MX, 0, MX as i64);
    nums.into_iter().zip(freq).map(|(num, freq)| {
        tree.update(num as i64, num as i64, &(freq as i64));
        tree.query(0, MX as i64)
    }).collect()
}

/// 懒删除堆
pub fn most_frequent_i_ds3(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
    let mut cnt = vec![0; nums.iter().max().copied().unwrap() as usize + 1];
    let mut heap = BinaryHeap::new();
    nums.into_iter().zip(freq).map(|(num, freq)| {
        cnt[num as usize] += freq as i64;
        heap.push((cnt[num as usize], num));
        while let Some(&(hc, hn)) = heap.peek() {
            if cnt[hn as usize] == hc { break; }
            heap.pop();
        }
        heap.peek().unwrap().0
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64>) {
        assert_eq!(func(vec![2, 3, 2, 1], vec![3, 2, -3, 1]), vec![3, 3, 2, 2]);
        assert_eq!(func(vec![5, 5, 3], vec![2, -2, 1]), vec![2, 0, 1]);
    }
    test(most_frequent_i_ds);
    test(most_frequent_i_ds2);
    test(most_frequent_i_ds3);
}
