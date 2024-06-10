//! 求出最长好子序列 I

pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k = k as usize;
    let mut dp = vec![vec![i32::MIN; k + 1]; len]; // dp[i][j] 表示以 nums[i] 结尾，已经有i次diff的最长好子序列长度
    for i in 0..len {
        dp[i][0] = 1;
    }
    let mut result = 1;
    for i in 0..len {
        for j in 0..i {
            for m in 0..=k {
                if nums[i] == nums[j] {
                    dp[i][m] = dp[i][m].max(dp[j][m] + 1);
                } else {
                    if m > 0 {
                        dp[i][m] = dp[i][m].max(dp[j][m - 1] + 1);
                    }
                }
                result = result.max(dp[i][m]);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2], 0), 1);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 1], 0), 2);
        assert_eq!(func(vec![1, 2, 1, 1, 3], 2), 4);
    }
    test(maximum_length);
}
