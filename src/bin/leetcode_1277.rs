//! 统计全为 1 的正方形子矩阵

pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    // dp[i][j] 表示以(i,j)为右下角的 最大矩阵
    let mut dp = vec![vec![0; n]; m];
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            dp[i][j] = if i == 0 || j == 0 {
                matrix[i][j]
            } else if matrix[i][j] == 0 {
                0
            } else {
                dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1
            };
            result += dp[i][j];
        }
    }
    result
}

fn main() {
    fn test(func: fn(matrix: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![
            vec![0, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1],
        ]), 15);
        assert_eq!(func(vec![
            vec![1, 0, 1],
            vec![1, 1, 0],
            vec![1, 1, 0],
        ]), 7);
    }
    test(count_squares);
}
