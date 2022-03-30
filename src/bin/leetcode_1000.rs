//! 合并石头的最低成本

/// f[i][j][k] = min(f[i][j][k], f[i][m][1] + f[m + 1][j][k - 1])
/// 为在[i, j]这个区间合并成k堆石子的最小成本
pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
    let len = stones.len();
    let k = k as usize;
    if (len - 1) % (k - 1) != 0 {
        return -1;
    }
    let mut dp = vec![vec![vec![i32::MAX / 2; k + 1]; len]; len];
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + stones[i];
    }
    for i in 0..len {
        dp[i][i][1] = 0;
    }
    for range in 2..=len {
        for i in 0..=len - range {
            let j = i + range - 1;
            for kk in 2..=k {
                for m in (i..j).step_by(k - 1) {
                    dp[i][j][kk] = dp[i][j][kk].min(dp[i][m][1] + dp[m + 1][j][kk - 1]);
                }
            }
            dp[i][j][1] = dp[i][j][k] + presum[j + 1] - presum[i];
        }
    }
    dp[0][len - 1][1]
}

fn main() {
    assert_eq!(merge_stones(vec![3, 2, 4, 1], 2), 20);
    assert_eq!(merge_stones(vec![3, 2, 4, 1], 3), -1);
    assert_eq!(merge_stones(vec![3, 5, 1, 2, 6], 3), 25);
}
