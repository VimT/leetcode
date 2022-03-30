//!和为K的子数组

use std::collections::HashMap;

/// nums 的 [i, j] 的和为 sum(nums[:j]) - sum(nums[:i-1])
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut ans = 0;
    let mut pre = 0;
    let mut map = HashMap::new();
    map.insert(0, 1);
    for i in 0..len {
        pre += nums[i];
        ans += map.get(&(pre - k)).copied().unwrap_or(0);
        *map.entry(pre).or_insert(0) += 1;
    }
    ans
}

fn main() {
    assert_eq!(subarray_sum(vec![28, 54, 7, -70, 22, 65, -6], 100), 1);
    assert_eq!(subarray_sum(vec![-1, -1, 1], 0), 1);
    assert_eq!(subarray_sum(vec![1], 0), 0);
    assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
}