//! 子数组不同元素数目的平方和 I

use std::collections::HashSet;

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        let mut cnt = HashSet::new();
        for j in i..len {
            cnt.insert(nums[j]);
            result = (result + cnt.len() * cnt.len()) % 1000000007;
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 1]), 15);
        assert_eq!(func(vec![2, 2]), 3);
    }
    test(sum_counts);
}
