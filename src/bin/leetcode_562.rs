//! 矩阵中最长的连续1线段

pub fn longest_line(mat: Vec<Vec<i32>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, -1), (-1, 0), (0, -1), (-1, 1)];
    let m = mat.len();
    let n = mat[0].len();
    let mut dp = vec![vec![vec![0; n]; m]; 8];

    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 {
                for k in 0..4 {
                    let (dx, dy) = DIR[k];
                    let (px, py) = (i as i32 + dx, j as i32 + dy);
                    let prev = if px >= 0 && px < m as i32 && py >= 0 && py < n as i32 {
                        dp[k][px as usize][py as usize]
                    } else { 0 };
                    dp[k][i][j] = prev + 1;
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 {
                result = result.max(dp[0][i][j]).max(dp[1][i][j]).max(dp[2][i][j]).max(dp[3][i][j]);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1]]), 3);
        assert_eq!(func(vec![vec![1, 1, 1, 1], vec![0, 1, 1, 0], vec![0, 0, 0, 1]]), 4);
    }
    test(longest_line);
}
