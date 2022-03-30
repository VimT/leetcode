//! 翻转矩阵后的得分

/// 先行变换，让最左边1列为1，再列变换，尽可能多1
pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        if grid[i][0] != 0 {
            for j in 0..n {
                grid[i][j] = if grid[i][j] == 0 { 1 } else { 0 };
            }
        }
    }
    for j in 0..n {
        let mut cnt = 0;
        for i in 0..m {
            if grid[i][j] == 1 { cnt += 1; }
        }
        if cnt <= m / 2 {
            for i in 0..m {
                grid[i][j] = if grid[i][j] == 0 { 1 } else { 0 };
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        let mut num = 0;
        for j in 0..n {
            num = num << 1 | grid[i][j];
        }
        result += num;
    }
    result
}

fn main() {
    assert_eq!(matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]), 39);
    assert_eq!(matrix_score(vec![vec![0]]), 1);
}
