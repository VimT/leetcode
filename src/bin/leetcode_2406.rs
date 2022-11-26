//! 将区间分为最少组数

use std::collections::{BinaryHeap, BTreeMap};

/// 差分
pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let max_len = intervals.iter().map(|x| x[1]).max().unwrap() as usize + 2;
    let mut diff = vec![0; max_len];
    for interval in intervals {
        diff[interval[0] as usize] += 1;
        diff[interval[1] as usize + 1] -= 1;
    }
    let mut cur = 0;
    let mut result = 0;
    for i in 0..max_len {
        cur += diff[i];
        result = result.max(cur);
    }
    result
}

/// 堆，堆顶存储每个组最后一个区间的 right
pub fn min_groups2(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable();
    let mut h: BinaryHeap<i32> = BinaryHeap::new();
    for interval in intervals {
        if !h.is_empty() && interval[0] > -*h.peek().unwrap() {
            h.pop();
        }
        h.push(-interval[1]);
    }
    h.len() as i32
}

/// 差分，btree
pub fn min_groups3(intervals: Vec<Vec<i32>>) -> i32 {
    let mut diff: BTreeMap<i32, i32> = BTreeMap::new();
    for interval in intervals {
        *diff.entry(interval[0]).or_default() += 1;
        *diff.entry(interval[1] + 1).or_default() -= 1;
    }
    let mut cur = 0;
    let mut result = 0;
    for (_, v) in diff {
        cur += v;
        result = result.max(cur);
    }
    result
}

fn main() {
    fn test(func: fn(intervals: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]]), 3);
        assert_eq!(func(vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]]), 1);
    }
    test(min_groups);
    test(min_groups2);
    test(min_groups3);
}
