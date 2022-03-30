//! 最大平均值和的分组

/// dp[k][i] 表示前i个数字分成k组的最大平均数和
pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let len = nums.len();
    let mut dp = vec![vec![0.0; len]; k];
    let mut cursum = 0;
    for i in 0..len {
        cursum += nums[i];
        dp[0][i] = cursum as f64 / (i + 1) as f64;
    }
    for i in 1..k {
        for j in i - 1..len {
            let mut cursum = 0;
            let mut tmp = 0f64;
            for k in (i..=j).rev() {
                cursum += nums[k];
                tmp = tmp.max(dp[i - 1][k - 1] + cursum as f64 / (j + 1 - k) as f64);
            }
            dp[i][j] = tmp;
        }
    }
    dp[k - 1][len - 1]
}

fn main() {
    assert_eq!(largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3), 20.0);
}

