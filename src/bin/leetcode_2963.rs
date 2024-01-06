//! 统计好分割方案的数目

use std::collections::HashMap;
use leetcode::algorithm::quick_pow;

pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    const MOD: i64 = 1_000_000_007;
    let mut group = 0;
    let mut last_seen: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        last_seen.insert(num, i + 1);
    }
    let mut i = 0;
    while i < len {
        let mut r = last_seen[&nums[i]];
        while i < r {
            r = r.max(last_seen[&nums[i]]);
            i += 1;
        }
        group += 1;
    }
    quick_pow(2, group - 1, MOD) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![13, 17, 76, 63, 98, 99, 88, 49, 17, 87, 21, 76, 5, 95, 23, 27, 71, 34, 61, 30, 51, 44, 84, 74, 81, 42, 16, 32, 26, 55, 16, 66, 7, 98, 55, 77, 83, 85, 80, 24, 40, 88, 5, 7, 4, 52, 64, 22, 21, 38, 28, 1, 5, 20, 42, 84, 5, 95, 14, 18, 75, 53, 57, 59, 34, 75, 10, 82, 65, 55, 20, 28, 9, 21, 28, 80, 7, 50, 52, 48, 86, 77, 80, 65, 48, 64, 9, 59, 41, 38, 69, 81, 63, 71, 51, 64, 26, 94, 77, 82]), 2);
        assert_eq!(func(vec![1, 5, 1, 5, 6]), 2);
        assert_eq!(func(vec![1, 2, 3, 4]), 8);
        assert_eq!(func(vec![1, 1, 1, 1]), 1);
        assert_eq!(func(vec![1, 2, 1, 3]), 2);
    }
    test(number_of_good_partitions);
}
