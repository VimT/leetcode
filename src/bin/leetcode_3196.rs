//! 最大化子数组的总成本

pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut dp = vec![0; len]; // dp[i] 表示以i结尾的的最大化子数组的总成本
    for i in 0..len {
        dp[i] = if i > 0 { dp[i - 1] } else { 0 } + nums[i] as i64;
        if i > 0 {
            dp[i] = dp[i].max(if i > 1 { dp[i - 2] } else { 0 } + nums[i - 1] as i64 - nums[i] as i64);
        }
    }
    dp[len - 1]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![1, -2, 3, 4]), 10);
        assert_eq!(func(vec![1, -1, 1, -1]), 4);
        assert_eq!(func(vec![0]), 0);
        assert_eq!(func(vec![1, -1]), 2);
    }
    test(maximum_total_cost);
}
