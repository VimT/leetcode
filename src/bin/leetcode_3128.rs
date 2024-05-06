//! 直角三角形

pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
    let m = grid.len();
    let n = grid[0].len();
    let mut column = vec![-1; n];
    let mut row = vec![-1; m];
    for i in 0..m {
        for j in 0..n {
            column[j] += grid[i][j] as i64;
            row[i] += grid[i][j] as i64;
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                result += row[i] * column[j];
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]]), 2);
        assert_eq!(func(vec![vec![1, 0, 0, 0], vec![0, 1, 0, 1], vec![1, 0, 0, 0]]), 0);
        assert_eq!(func(vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]]), 2);
    }
    test(number_of_right_triangles);
}
