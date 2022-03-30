//! 不同路径 II


pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    fn inner(grid: &Vec<Vec<i32>>, x: usize, y: usize, m: usize, n: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if x == m - 1 && y == n - 1 { return 1; }
        if cache[x][y] != -1 { return cache[x][y]; }
        let mut ans = 0;
        if x + 1 < m && grid[x + 1][y] == 0 {
            ans += inner(grid, x + 1, y, m, n, cache);
        }
        if y + 1 < n && grid[x][y + 1] == 0 {
            ans += inner(grid, x, y + 1, m, n, cache);
        }
        cache[x][y] = ans;
        ans
    }
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    inner(&obstacle_grid, 0, 0, m, n, &mut vec![vec![-1; n]; m])
}

/// dp[i][j] 是到达 i, j 有多少条路径
/// dp[i][j] = dp[i-1][j] + dp[i][j-1]
pub fn unique_paths_with_obstacles_dp(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    dp[0][1] = 1;
    for i in 1..=m {
        for j in 1..=n {
            if obstacle_grid[i - 1][j - 1] == 0 {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }
    dp[m][n]
}

fn main() {
    fn test(func: fn(obstacle_grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), 2);
        assert_eq!(func(vec![vec![0, 1], vec![0, 0]]), 1);
    }
    test(unique_paths_with_obstacles);
    test(unique_paths_with_obstacles_dp);
}
