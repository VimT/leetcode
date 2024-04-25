//! 使矩阵满足条件的最少操作次数

pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![[usize::MAX; 10]; n + 1]; // d[i][j] 表示前i列，最后一列是j的数量，dp[i][j] = dp[i-1][k] + grid[n][i] to j (k!=j)
    dp[0] = [0; 10];
    for i in 0..n {
        let mut cnt = [0; 10];
        for j in 0..m {
            cnt[grid[j][i] as usize] += 1;
        }
        for j in 0..10 {
            let need = m - cnt[j];
            for k in 0..10 {
                if j != k {
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][k] + need);
                }
            }
        }
    }
    dp[n].iter().min().copied().unwrap() as i32
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 0, 2], vec![1, 0, 2]]), 0);
        assert_eq!(func(vec![vec![1, 1, 1], vec![0, 0, 0]]), 3);
        assert_eq!(func(vec![vec![1], vec![2], vec![3]]), 2);
    }
    test(minimum_operations);
}
