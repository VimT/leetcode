//! 边界着色

use std::collections::VecDeque;

pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let row = row as usize;
    let col = col as usize;
    let origin_color = grid[row][col];
    let mut vis = vec![vec![false; n]; m];
    let mut q = VecDeque::new();
    q.push_back((row, col));
    vis[row][col] = true;
    let dir = [-1, 0, 1, 0, -1];
    let mut boards = vec![];
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        let mut is_border = false;
        for i in 0..4 {
            let nx = x as i32 + dir[i];
            let ny = y as i32 + dir[i + 1];
            if !(nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] == origin_color) {
                is_border = true;
            } else if !vis[nx as usize][ny as usize] {
                vis[nx as usize][ny as usize] = true;
                q.push_back((nx as usize, ny as usize));
            }
        }
        if is_border {
            boards.push((x, y));
        }
    }
    for (x, y) in boards {
        grid[x][y] = color;
    }
    grid
}

fn main() {
    assert_eq!(color_border(vec![vec![1, 1], vec![1, 2]], 0, 0, 3), vec![vec![3, 3], vec![3, 2]]);
    assert_eq!(color_border(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3), vec![vec![1, 3, 3], vec![2, 3, 3]]);
    assert_eq!(color_border(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2), vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]);
}

