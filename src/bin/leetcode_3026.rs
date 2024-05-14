//! 最大好子数组和

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut result = i64::MIN;
    let mut seen: HashMap<i32, usize> = HashMap::new();
    seen.insert(nums[0], 0);
    for i in 1..len {
        if let Some(&j) = seen.get(&(nums[i] - k)) {
            result = result.max(presum[i + 1] - presum[j]);
        }
        if let Some(&j) = seen.get(&(nums[i] + k)) {
            result = result.max(presum[i + 1] - presum[j]);
        }
        match seen.entry(nums[i]) {
            Entry::Occupied(mut v) => {
                if presum[*v.get()] > presum[i] {
                    *v.get_mut() = i;
                }
            }
            Entry::Vacant(v) => {
                v.insert(i);
            }
        }
    }

    if result == i64::MIN { 0 } else { result }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6], 1), 11);
        assert_eq!(func(vec![-1, 3, 2, 4, 5], 3), 11);
        assert_eq!(func(vec![-1, -2, -3, -4], 2), -6);
    }
    test(maximum_subarray_sum);
}
