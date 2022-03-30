//! 最小路径和

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = grid;
    for i in 1..m {
        dp[i][0] += dp[i - 1][0];
    }
    for i in 1..n {
        dp[0][i] += dp[0][i - 1];
    }
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + dp[i][j];
        }
    }
    dp[m - 1][n - 1]
}

fn main() {
    assert_eq!(min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
    assert_eq!(min_path_sum(vec![vec![1]]), 1);
    assert_eq!(min_path_sum(vec![vec![1, 2]]), 3);
}

