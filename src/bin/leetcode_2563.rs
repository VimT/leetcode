//! 统计公平数对的数目

use leetcode::algorithm::{binary_search_lower, binary_search_upper};

/// 直接二分。或者双指针，但是双指针很不好写
pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = 0;
    for r in 0..len {
        let l1 = binary_search_lower(&nums[0..r], &(lower - nums[r]));
        let l2 = binary_search_upper(&nums[0..r], &(upper - nums[r]));
        result += l2 - l1;
    }
    result as i64
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, lower: i32, upper: i32) -> i64) {
        assert_eq!(func(vec![0, 0, 0, 0, 0, 0], 0, 0), 15);
        assert_eq!(func(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
        assert_eq!(func(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
    test(count_fair_pairs);
}
