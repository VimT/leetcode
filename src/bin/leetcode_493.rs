//! 翻转对

use std::collections::{BTreeMap, BTreeSet, HashMap};

use leetcode::algorithm::BinIndexedTree;

// timeout
pub fn reverse_pairs_btreemap(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = 0;
    let mut m: BTreeMap<i64, i32> = BTreeMap::new();
    for i in 0..len {
        for (_, v) in m.range((2 * nums[i] as i64 + 1)..) {
            result += *v;
        }
        *m.entry(nums[i] as i64).or_default() += 1;
    }
    result as i32
}

pub fn reverse_pairs_merge_sort(mut nums: Vec<i32>) -> i32 {
    fn merge(nums: &mut [i32]) -> usize {
        if nums.len() <= 1 { return 0; }
        let len = nums.len();
        let mid = len / 2;
        let left = merge(&mut nums[0..mid]);
        let right = merge(&mut nums[mid..]);
        let mut result = left + right;
        nums[0..mid].sort_unstable();
        nums[mid..].sort_unstable();
        let mut i = 0;
        for j in mid..len {
            while i < mid && nums[i] as i64 <= 2 * nums[j] as i64 {
                i += 1;
            }
            result += mid - i;
        }
        result
    }
    merge(&mut nums) as i32
}

/// 树状数组
pub fn reverse_pairs_szsz(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    // 利用哈希表进行离散化
    let mut set = BTreeSet::new();
    for &x in &nums {
        set.insert(x as i64);
        set.insert(x as i64 * 2);
    }
    let map: HashMap<i64, usize> = set.into_iter().enumerate().map(|(idx, val)| (val, idx + 1)).collect();
    let mlen = map.len();
    let mut tree = BinIndexedTree::with_capacity(mlen + 1);
    let mut result = 0;
    for i in 0..len {
        let left = map[&(nums[i] as i64 * 2)];
        let right = mlen;
        result += tree.sum(right) - tree.sum(left);
        tree.add(map[&(nums[i] as i64)], 1);
    }
    result
}

fn main() {
    fn assert(nums: Vec<i32>, result: i32) {
        assert_eq!(reverse_pairs_btreemap(nums.clone()), result);
        assert_eq!(reverse_pairs_merge_sort(nums.clone()), result);
        assert_eq!(reverse_pairs_szsz(nums.clone()), result);
    }
    assert(vec![2, 4, 3, 5, 1], 3);
    assert(vec![1, 3, 2, 3, 1], 2);
}
