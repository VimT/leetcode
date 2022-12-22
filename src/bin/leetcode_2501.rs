//! 数组中最长的方波

use std::collections::HashSet;

/// 排序+二分查找
pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = -1;
    let end = (i32::MAX as f64).sqrt() as i32;
    for mut i in 0..len {
        if nums[i] > end || nums[i] * nums[i] > nums[len - 1] { break; }
        let mut cur = 1;
        while let Ok(ni) = nums.binary_search(&(nums[i] * nums[i])) {
            i = ni;
            cur += 1;
        }
        if cur >= 2 {
            result = result.max(cur);
        }
    }
    result
}

/// hashset
pub fn longest_square_streak2(nums: Vec<i32>) -> i32 {
    let nums: HashSet<i32> = nums.into_iter().collect();
    let mut result = -1;
    for &num in &nums {
        let mut x = num * num;
        let mut cnt = 1;
        while nums.contains(&x) {
            x = x * x;
            cnt += 1;
        }
        if cnt >= 2 {
            result = result.max(cnt)
        }
    }
    result
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 3, 6, 16, 8, 2]), 3);
        assert_eq!(func(vec![2, 3, 5, 6, 7]), -1);
    }
    test(longest_square_streak);
    test(longest_square_streak2);
}
