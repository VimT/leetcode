//! 和等于 k 的最长子数组长度

use std::collections::HashMap;

pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
    let mut pos = HashMap::new();
    let len = nums.len();
    let mut result = 0;
    let mut cursum = 0;
    pos.insert(0, 0);
    for i in 0..len {
        cursum += nums[i];
        if let Some(&pre_idx) = pos.get(&(cursum - k)) {
            result = result.max(i + 1 - pre_idx);
        }
        if !pos.contains_key(&cursum) {
            pos.insert(cursum, i + 1);
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, -1, 5, -2, 3], 3), 4);
        assert_eq!(func(vec![-2, -1, 2, 1], 1), 2);
    }
    test(max_sub_array_len);
}
