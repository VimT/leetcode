//! 不间断子数组


use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut result = 0;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut l = 0;
    for r in 0..len {
        *cnt.entry(nums[r]).or_default() += 1;
        loop {
            let mn = *cnt.keys().min().unwrap();
            let mx = *cnt.keys().max().unwrap();
            if mx - mn <= 2 { break; }
            if let Entry::Occupied(mut v) = cnt.entry(nums[l]) {
                *v.get_mut() -= 1;
                if *v.get() == 0 { v.remove(); }
            }
            l += 1;
        }
        result += r - l + 1;
    }
    result as i64
}

/// 单调队列维护区间的最大值和最小值
pub fn continuous_subarrays2(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut l = 0;
    let mut dec = VecDeque::with_capacity(len);
    let mut inc = VecDeque::with_capacity(len);
    let mut result = len;
    for r in 0..len {
        while !dec.is_empty() && nums[r] > *dec.back().unwrap() {
            dec.pop_back();
        }
        while !inc.is_empty() && nums[r] < *inc.back().unwrap() {
            inc.pop_back();
        }
        dec.push_back(nums[r]);
        inc.push_back(nums[r]);
        while l < r && *dec.front().unwrap() - *inc.front().unwrap() > 2 {
            if nums[l] == *dec.front().unwrap() { dec.pop_front(); }
            if nums[l] == *inc.front().unwrap() { inc.pop_front(); }
            l += 1;
        }
        if *dec.front().unwrap() - *inc.front().unwrap() <= 2 {
            result += r - l;
        }
    }
    result as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![31, 30, 31, 32]), 10);
        assert_eq!(func(vec![5, 4, 2, 4]), 8);
        assert_eq!(func(vec![1, 2, 3]), 6);
    }
    test(continuous_subarrays);
}
