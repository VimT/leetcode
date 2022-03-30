//! 统计农场中肥沃金字塔的数目

pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut pre_sums = vec![vec![0; n + 1]; m];
    for i in 0..m {
        for j in 0..n {
            pre_sums[i][j + 1] = pre_sums[i][j] + grid[i][j];
        }
    }
    let mut result = 0;
    let mut dp = vec![vec![0; n]; m];
    for i in (0..m - 1).rev() {
        for j in 1..n - 1 {
            if grid[i][j] == 1 && grid[i + 1][j] == 1 && grid[i + 1][j - 1] == 1 && grid[i + 1][j + 1] == 1 {
                dp[i][j] = dp[i + 1][j - 1].min(dp[i + 1][j + 1]) + 1;
                result += dp[i][j];
            }
        }
    }
    dp = vec![vec![0; n]; m];
    for i in 1..m {
        for j in 1..n - 1 {
            if grid[i][j] == 1 && grid[i - 1][j] == 1 && grid[i - 1][j - 1] == 1 && grid[i - 1][j + 1] == 1 {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j + 1]) + 1;
                result += dp[i][j];
            }
        }
    }
    result
}

fn main() {
    assert_eq!(count_pyramids(vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]]), 2);
    assert_eq!(count_pyramids(vec![vec![1, 1, 1], vec![1, 1, 1]]), 2);
    assert_eq!(count_pyramids(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]), 0);
    assert_eq!(count_pyramids(vec![vec![1, 1, 1, 1, 0], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![0, 1, 0, 0, 1]]), 13);
}
