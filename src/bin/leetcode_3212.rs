//! 统计 X 和 Y 频数相等的子矩阵数量

pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut x_sum = vec![vec![0; n + 1]; m + 1];
    let mut y_sum = vec![vec![0; n + 1]; m + 1];

    let mut result = 0;
    // 二维前缀和
    for i in 1..=m {
        for j in 1..=n {
            x_sum[i][j] = x_sum[i - 1][j] + x_sum[i][j - 1] - x_sum[i - 1][j - 1] + (grid[i - 1][j - 1] == 'X') as i32;
            y_sum[i][j] = y_sum[i - 1][j] + y_sum[i][j - 1] - y_sum[i - 1][j - 1] + (grid[i - 1][j - 1] == 'Y') as i32;
            if x_sum[i][j] > 0 && x_sum[i][j] == y_sum[i][j] {
                result += 1;
            }
        }
    }
    result
}

/// 行遍历，同时维护每列的个数
pub fn number_of_submatrices2(grid: Vec<Vec<char>>) -> i32 {
    let n = grid[0].len();
    let mut result = 0;
    let mut col_x = vec![0; n];
    let mut col_y = vec![0; n];
    for row in grid {
        let mut cnt_x = 0;
        let mut cnt_y = 0;
        for (j, ch) in row.into_iter().enumerate() {
            if ch == 'X' { col_x[j] += 1; } else if ch == 'Y' { col_y[j] += 1; }
            cnt_x += col_x[j];
            cnt_y += col_y[j];
            if cnt_x > 0 && cnt_x == cnt_y {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<char>>) -> i32) {
        assert_eq!(func(vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']]), 3);
        assert_eq!(func(vec![vec!['X', 'X'], vec!['X', 'Y']]), 0);
        assert_eq!(func(vec![vec!['.', '.'], vec!['.', '.']]), 0);
    }
    test(number_of_submatrices);
    test(number_of_submatrices2);
}
