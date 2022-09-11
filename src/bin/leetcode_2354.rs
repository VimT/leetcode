//! 优质数对的数目

use std::collections::HashSet;

pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
    let mut cnt = [0; 32];
    let mut set = HashSet::with_capacity(nums.len());
    for num in nums {
        if set.insert(num) {
            cnt[num.count_ones() as usize] += 1;
        }
    }
    let mut result = 0;
    for i in 0..32 {
        for j in 0..32 {
            if i + j >= k as usize {
                result += cnt[i] * cnt[j];
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![1, 2, 3, 1], 3), 5);
        assert_eq!(func(vec![5, 1, 1], 10), 0);
    }
    test(count_excellent_pairs);
}
