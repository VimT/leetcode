//! 求出最长好子序列 II

use std::collections::HashMap;

/// dp 优化：要么是延续相同的数，要么是数字变了
pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k = k as usize;
    let mut dp = vec![vec![i32::MIN; k + 1]; len + 1]; // dp[i][j] 表示以 nums[i-1] 结尾，错 j 次的最长好子序列长度
    dp[0].fill(0);
    let mut cur = vec![0; k + 1]; // 表示当前错 i 次的最长好子序列长度
    let mut last_seen: HashMap<i32, usize> = HashMap::new(); // 记录上一次出现的位置
    let mut result = 1;
    for i in 1..=len {
        let before = last_seen.get(&nums[i - 1]).copied().unwrap_or(0);
        for j in 0..=k { dp[i][j] = dp[before][j] + 1 }
        for j in 1..=k { dp[i][j] = dp[i][j].max(cur[j - 1] + 1); }
        last_seen.insert(nums[i - 1], i);
        for j in 0..=k {
            cur[j] = dp[i][j].max(cur[j]);
            result = result.max(dp[i][j]);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![89, 89, 90, 88, 88, 88, 88, 90, 90], 2), 8);
        assert_eq!(func(vec![38, 40, 38, 38], 2), 4);
        assert_eq!(func(vec![29, 30, 30], 1), 3);
        assert_eq!(func(vec![1, 2, 1, 1, 3], 2), 4);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }
    test(maximum_length);
}
