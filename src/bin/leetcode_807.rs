//! 保持城市天际线

pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut row_max = vec![0; len];
    let mut col_max = vec![0; len];
    for i in 0..len {
        for j in 0..len {
            row_max[i] = row_max[i].max(grid[i][j]);
            col_max[j] = col_max[j].max(grid[i][j]);
        }
    }
    let mut result = 0;
    for i in 0..len {
        for j in 0..len {
            result += row_max[i].min(col_max[j]) - grid[i][j];
        }
    }
    result
}

fn main() {
    assert_eq!(max_increase_keeping_skyline(vec![vec![3, 0, 8, 4], vec![2, 4, 5, 7], vec![9, 2, 6, 3], vec![0, 3, 1, 0]]), 35);
    assert_eq!(max_increase_keeping_skyline(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]), 0);
}
