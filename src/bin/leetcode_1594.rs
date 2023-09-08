//! 矩阵的最大非负积

pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![[0, 0]; n]; m];
    dp[0][0] = [grid[0][0] as i64, grid[0][0] as i64];
    for i in 1..n {
        dp[0][i][0] = dp[0][i - 1][0] * grid[0][i] as i64;
        dp[0][i][1] = dp[0][i - 1][1] * grid[0][i] as i64;
    }
    for i in 1..m {
        dp[i][0][0] = dp[i - 1][0][0] * grid[i][0] as i64;
        dp[i][0][1] = dp[i - 1][0][1] * grid[i][0] as i64;
    }
    for i in 1..m {
        for j in 1..n {
            let num = grid[i][j] as i64;
            if num > 0 {
                dp[i][j][0] = dp[i - 1][j][0].max(dp[i][j - 1][0]) * num;
                dp[i][j][1] = dp[i - 1][j][1].min(dp[i][j - 1][1]) * num;
            } else if num < 0 {
                dp[i][j][0] = dp[i - 1][j][1].min(dp[i][j - 1][1]) * num;
                dp[i][j][1] = dp[i - 1][j][0].max(dp[i][j - 1][0]) * num;
            }
        }
    }
    if dp[m - 1][n - 1][0] < 0 { -1 } else { (dp[m - 1][n - 1][0] % (1e9 as i64 + 7)) as i32 }
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]]), -1);
        assert_eq!(func(vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]]), 8);
        assert_eq!(func(vec![vec![1, 3], vec![0, -4]]), 0);
    }
    test(max_product_path);
}
