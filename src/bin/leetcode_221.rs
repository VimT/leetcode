//! 最大正方形

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut max_side = 0;
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return 0;
    }
    let rows = matrix.len();
    let columns = matrix[0].len();
    let mut dp = vec![vec![0; columns]; rows];
    for i in 0..rows {
        for j in 0..columns {
            if matrix[i][j] == '1' {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
                max_side = max_side.max(dp[i][j]);
            }
        }
    }
    max_side * max_side
}


fn main() {
    assert_eq!(maximal_square(vec![vec!['1', '0', '1', '0', '0'], vec!['1', '0', '1', '1', '1'], vec!['1', '1', '1', '1', '1'], vec!['1', '0', '0', '1', '0']]), 4);
    assert_eq!(maximal_square(vec![vec!['0', '1'], vec!['1', '0']]), 1);
    assert_eq!(maximal_square(vec![vec!['0']]), 0);
}
