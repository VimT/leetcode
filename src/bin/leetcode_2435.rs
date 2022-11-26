//! 矩阵中和能被 K 整除的路径



/// 查表法：dp[i+1][j+1][v] = dp[i+1][j][(v-grid[i][j]) % k] + dp[i][j+1][(v-grid[i][j] % k]
/// 刷表法：dp[i+1][j+1][(v+grid[i][j]) % k] = dp[i+1][j][v] + dp[i][j+1][v]
pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let m = grid.len();
    let n = grid[0].len();
    let k = k as usize;
    let mut dp = vec![vec![vec![0; k]; n + 1]; m + 1];
    dp[0][1][0] = 1;
    for i in 0..m {
        for j in 0..n {
            let x = grid[i][j] as usize;
            for v in 0..k {
                dp[i + 1][j + 1][(v + x) % k] = (dp[i + 1][j][v] + dp[i][j + 1][v]) % MOD;
            }
        }
    }
    dp[m][n][0]
}

pub fn number_of_paths2(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let m = grid.len();
    let n = grid[0].len();
    let k = k as usize;
    fn dfs(grid: &Vec<Vec<i32>>, k: usize, i: usize, j: usize, v: usize, cache: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        let v = (v + grid[i][j] as usize) % k;
        if cache[i][j][v] != -1 { return cache[i][j][v]; }
        if i == 0 && j == 0 { return (v == 0) as i32; }
        let mut result = 0;
        if i > 0 {
            result += dfs(grid, k, i - 1, j, v, cache);
        }
        if j > 0 {
            result += dfs(grid, k, i, j - 1, v, cache);
        }
        result %= MOD;
        cache[i][j][v] = result;
        return result;
    }
    dfs(&grid, k, m - 1, n - 1, 0, &mut vec![vec![vec![-1; k]; n]; m])
}


fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![1,5,3,7,3,2,3,5]], 29), 1);
        assert_eq!(func(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3), 2);
        assert_eq!(func(vec![vec![0, 0]], 5), 1);
        assert_eq!(func(vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]], 1), 10);
    }
    test(number_of_paths);
    test(number_of_paths2);
}
