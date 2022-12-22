//! 统计中位数为 K 的子数组

use std::cmp::Ordering;
use std::collections::HashMap;

/// 中位数： >k的置为1，<k的置为0，k为中位数的子数组和为0或者1
pub fn count_subarrays(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut ki = 0;
    for (i, num) in nums.iter_mut().enumerate() {
        *num = match k.cmp(num) {
            Ordering::Less => 1,
            Ordering::Equal => {
                ki = i;
                0
            }
            Ordering::Greater => -1
        }
    }
    let len = nums.len();
    let mut presum = HashMap::new();
    presum.insert(0, 1);
    let mut cursum = 0;
    for i in ki + 1..len {
        cursum += nums[i];
        *presum.entry(cursum).or_default() += 1;
    }

    let mut result = presum.get(&0).cloned().unwrap_or(0) + presum.get(&1).cloned().unwrap_or(0);
    cursum = 0;
    for i in (0..ki).rev() {
        cursum += -nums[i];
        result += presum.get(&cursum).cloned().unwrap_or(0) + presum.get(&(cursum + 1)).cloned().unwrap_or(0);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![3, 2, 1, 4, 5], 4), 3);
        assert_eq!(func(vec![2, 3, 1], 3), 1);
    }
    test(count_subarrays);
}
