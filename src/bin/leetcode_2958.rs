//! 最多 K 个重复元素的最长子数组

use std::collections::{HashMap, VecDeque};

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut cnt: HashMap<i32, VecDeque<usize>> = HashMap::new();
    let mut l = 0;
    let k = k as usize;
    let mut result = 0;
    for r in 1..=len {
        let v = cnt.entry(nums[r - 1]).or_default();
        v.push_back(r);
        if v.len() > k { l = l.max(v.pop_front().unwrap()); }
        result = result.max(r - l);
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 5, 5], 1), 2);
        assert_eq!(func(vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
        assert_eq!(func(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
        assert_eq!(func(vec![5, 5, 5, 5, 5, 5, 5], 4), 4);
    }
    test(max_subarray_length);
}
