//! 统计趣味子数组的数目

use std::collections::HashMap;

pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    let mut cnt_mod: HashMap<i32, i64> = HashMap::new();
    cnt_mod.insert(0, 1);
    let mut cnt = 0;
    let mut result = 0;
    for num in nums.into_iter() {
        if num % modulo == k {
            cnt += 1;
        }
        if let Some(&j) = cnt_mod.get(&((cnt + modulo - k) % modulo)) {
            result += j;
        }
        *cnt_mod.entry(cnt % modulo).or_default() += 1;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, modulo: i32, k: i32) -> i64) {
        assert_eq!(func(vec![3, 2, 4], 2, 1), 3);
        assert_eq!(func(vec![3, 1, 9, 6], 3, 0), 2);
    }
    test(count_interesting_subarrays);
}
