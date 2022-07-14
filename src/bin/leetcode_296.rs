//! 最佳的碰头地点

/// 从一维来看，最佳位置是 中位数，所以分别对行列进行计算
pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
    let mut cols = vec![];
    let mut rows = vec![];
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                rows.push(i);
                cols.push(j);
            }
        }
    }
    if cols.is_empty() { return 0; }
    cols.sort_unstable();
    let col_mid = cols[cols.len() / 2];
    let row_mid = rows[rows.len() / 2];
    let mut result = 0;
    for col in cols {
        result += (col as i32 - col_mid as i32).abs();
    }
    for row in rows {
        result += (row as i32 - row_mid as i32).abs();
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 0, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0]]), 6);
        assert_eq!(func(vec![vec![1, 1]]), 1);
    }
    test(min_total_distance);
}
