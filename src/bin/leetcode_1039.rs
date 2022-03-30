//! 多边形三角剖分的最低得分

/// dp[i][j]表示从i到j序列的最低分
/// dp[i][j]=min(dp[i][m]+A[i]*A[j]*A[m]+dp[m][j]),for m in range [i+1,j-1]
pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let len = values.len();
    let mut dp = vec![vec![0; len]; len];
    for i in (0..len - 2).rev() {
        for j in i + 2..len {
            for k in i + 1..j {
                let tmp = values[i] * values[j] * values[k] + dp[i][k] + dp[k][j];
                if dp[i][j] == 0 {
                    dp[i][j] = tmp;
                } else {
                    dp[i][j] = dp[i][j].min(tmp);
                }
            }
        }
    }
    dp[0][len - 1]
}

fn main() {
    assert_eq!(min_score_triangulation(vec![1, 2, 3]), 6);
    assert_eq!(min_score_triangulation(vec![3, 7, 4, 5]), 144);
    assert_eq!(min_score_triangulation(vec![1, 3, 1, 4, 1, 5]), 13);
}
