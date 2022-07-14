//! 判断矩阵是否是一个 X 矩阵

pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    for i in 0..m {
        for j in 0..m {
            if (i as i32 - j as i32) == 0 || (i + j) == m - 1 {
                if grid[i][j] == 0 { return false; }
            } else {
                if grid[i][j] != 0 { return false; }
            }
        }
    }
    true
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![2, 0, 0, 1], vec![0, 3, 1, 0], vec![0, 5, 2, 0], vec![4, 0, 0, 2]]), true);
        assert_eq!(func(vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]]), false);
    }
    test(check_x_matrix);
}
