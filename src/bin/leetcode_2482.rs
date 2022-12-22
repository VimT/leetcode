//! 行和列中一和零的差值


pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut one_row = vec![0; m];
    let mut one_col = vec![0; n];
    let mut zero_row = vec![0; m];
    let mut zero_col = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                zero_row[i] += 1;
                zero_col[j] += 1;
            } else {
                one_row[i] += 1;
                one_col[j] += 1;
            }
        }
    }
    let mut diff = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            diff[i][j] = one_row[i] + one_col[j] - zero_row[i] - zero_col[j];
        }
    }
    diff
}

/// 空间优化，是1加+1，是0就-1
pub fn ones_minus_zeros2(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut row = vec![0; m];
    let mut col = vec![0; n];
    for (i, r) in grid.iter().enumerate() {
        for (j, &num) in r.iter().enumerate() {
            row[i] += num * 2 - 1;  // 不用if
            col[j] += num * 2 - 1;
        }
    }
    for (i, r) in grid.iter_mut().enumerate() {
        for (j, num) in r.iter_mut().enumerate() {
            *num = row[i] + col[j];
        }
    }
    grid
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]), vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]);
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 1, 1]]), vec![vec![5, 5, 5], vec![5, 5, 5]]);
    }
    test(ones_minus_zeros);
    test(ones_minus_zeros2);
}
