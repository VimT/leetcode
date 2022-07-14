//! 网格中的最小路径代价

pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![i32::MAX; n]; m];
    for i in 0..n {
        dp[0][i] = grid[0][i];
    }
    for i in 1..m {
        for j in 0..n {
            for k in 0..n {
                dp[i][j] = dp[i][j].min(dp[i - 1][k] + move_cost[grid[i - 1][k] as usize][j] + grid[i][j]);
            }
        }
    }
    *dp[m - 1].iter().min().unwrap()
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![5, 3], vec![4, 0], vec![2, 1]], vec![vec![9, 8], vec![1, 5], vec![10, 12], vec![18, 6], vec![2, 4], vec![14, 3]]), 17);
        assert_eq!(func(vec![vec![5, 1, 2], vec![4, 0, 3]], vec![vec![12, 10, 15], vec![20, 23, 8], vec![21, 7, 1], vec![8, 1, 13], vec![9, 10, 25], vec![5, 3, 2]]), 6);
    }
    test(min_path_cost);
}
