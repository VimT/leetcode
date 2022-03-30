//! 二进制矩阵中的最短路径


use std::collections::VecDeque;

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 { return -1; }
    let mut result = vec![vec![-1; n]; n];
    let n = grid.len();
    let mut visited = vec![vec![false; n]; n];
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    result[0][0] = 1;
    visited[0][0] = true;
    while !q.is_empty() {
        let (r, c) = q.pop_front().unwrap();
        for i in r as i32 - 1..=r as i32 + 1 {
            for j in c as i32 - 1..=c as i32 + 1 {
                let (i, j) = (i as usize, j as usize);
                if i < n && j < n && grid[i][j] == 0 && !visited[i][j] {
                    visited[i][j] = true;
                    q.push_back((i, j));
                    result[i][j] = result[r][c] + 1;
                }
            }
        }
    }
    result[n - 1][n - 1]
}


fn main() {
    assert_eq!(shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]), 2);
    assert_eq!(shortest_path_binary_matrix(vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]]), 4);
    assert_eq!(shortest_path_binary_matrix(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]]), -1);
}
