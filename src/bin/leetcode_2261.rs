//! 含最多 K 个可整除元素的子数组

use std::collections::HashSet;

pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
    let mut result = HashSet::new();
    const MOD: u64 = 1e9 as u64 + 7;
    const BASE1: u64 = 7;
    const BASE2: u64 = 13;
    let len = nums.len();
    for i in 0..len {
        let mut cnt = 0;
        let mut num1 = 0;
        let mut num2 = 0;
        for j in i..len {
            if nums[j] % p == 0 {
                cnt += 1;
                if cnt > k { break; }
            }
            num1 = (num1 * BASE1 + nums[j] as u64) % MOD;
            num2 = (num2 * BASE2 + nums[j] as u64) % MOD;
            result.insert((num1, num2));
        }
    }
    result.len() as i32
}

fn main() {
    assert_eq!(count_distinct(vec![10, 2, 17, 7, 20], 1, 10), 14);
    assert_eq!(count_distinct(vec![2, 3, 3, 2, 2], 2, 2), 11);
    assert_eq!(count_distinct(vec![1, 2, 3, 4], 4, 1), 10);
}
