//! 求出所有子序列的能量和

use leetcode::algorithm::quick_pow;

pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let len = nums.len();
    let mut dp = vec![vec![0; 101]; len + 1];  // dp[i][j] 表示前i个数中子序列和为k的个数
    for i in 0..=len {
        dp[i][0] = quick_pow(2, i as i64, MOD);
    }
    for i in 0..len {
        for j in 1..101 {
            dp[i + 1][j] = (dp[i][j] * 2 + if nums[i] as usize <= j { dp[i][j - nums[i] as usize] } else { 0 }) % MOD;
        }
    }
    return dp[len][k as usize] as i32;
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3], 3), 6);
        assert_eq!(func(vec![2, 3, 3], 5), 4);
        assert_eq!(func(vec![1, 2, 3], 7), 0);
    }
    test(sum_of_power);
}
