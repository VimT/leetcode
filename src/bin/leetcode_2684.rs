//! 矩阵中移动的最大次数

pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    static DIR: [(i32, i32); 3] = [(-1, 1), (0, 1), (1, 1)];
    let mut dp = vec![vec![0; n]; m];
    for j in (0..n - 1).rev() {
        for i in 0..m {
            for (dx, dy) in DIR {
                let (nx, ny) = ((i as i32 + dx) as usize, (j as i32 + dy) as usize);
                if nx < m && ny < n {
                    if grid[nx][ny] > grid[i][j] {
                        dp[i][j] = dp[i][j].max(dp[nx][ny] + 1);
                    }
                }
            }
        }
    }
    dp.iter().map(|x| x[0]).max().unwrap()
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![2, 4, 3, 5], vec![5, 4, 9, 3], vec![3, 4, 2, 11], vec![10, 9, 13, 15]]), 3);
        assert_eq!(func(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]]), 0);
    }
    test(max_moves);
}
