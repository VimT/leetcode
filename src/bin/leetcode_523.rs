//! 连续的子数组和


use std::collections::HashMap;

/// 当 prefixSums[q]−prefixSums[p] 为 k 的倍数时，prefixSums[p] 和 prefixSums[q] 除以 kk 的余数相同
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    if len < 2 { return false; }
    let k = k as i64;
    let mut cur_sum = 0;
    let mut set = HashMap::new();
    set.insert(0, 0);
    for i in 0..len {
        cur_sum = (cur_sum + nums[i] as i64) % k;
        if let Some(&v) = set.get(&cur_sum) {
            if i + 1 - v >= 2 { return true; }
        } else {
            set.insert(cur_sum, i + 1);
        }
    }
    false
}

fn main() {
    assert_eq!(check_subarray_sum(vec![1, 1], 2), true);
    assert_eq!(check_subarray_sum(vec![0], 1), false);
    assert_eq!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
    assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 13), false);
}
