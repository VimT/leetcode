//! 使数组变美的最小增量运算数

pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut dp = vec![i64::MAX; len + 1]; // dp[i] 表示第i-1个元素>=k时，前i个元素最小递增数
    dp[0] = 0;
    for i in 1..=len {
        let prev_min = *dp[i.saturating_sub(3)..i].iter().min().unwrap();
        dp[i] = prev_min + (k - nums[i - 1]).max(0) as i64;
    }
    *dp[len - 2..=len].iter().min().unwrap()
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![13, 34, 0, 13, 9, 19], 82), 117);
        assert_eq!(func(vec![4, 0, 10, 2, 10, 6], 8), 0);
        assert_eq!(func(vec![2, 3, 0, 0, 2], 4), 3);
        assert_eq!(func(vec![0, 1, 3, 3], 5), 2);
        assert_eq!(func(vec![1, 1, 2], 1), 0);
    }
    test(min_increment_operations);
}
