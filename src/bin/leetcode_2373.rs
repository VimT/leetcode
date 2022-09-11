//! 矩阵中的局部最大值


pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut result = vec![vec![0; n - 2]; n - 2];
    for i in 0..n - 2 {
        for j in 0..n - 2 {
            let mut max = 0;
            for x in 0..3 {
                for y in 0..3 {
                    max = max.max(grid[i + x][j + y]);
                }
            }
            result[i][j] = max;
        }
    }
    result
}

fn main() {
    assert_eq!(largest_local(vec![vec![9, 9, 8, 1], vec![5, 6, 2, 6], vec![8, 2, 6, 4], vec![6, 2, 2, 2]]), vec![vec![9, 9], vec![8, 6]]);
    assert_eq!(largest_local(vec![vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 2, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]);
}
