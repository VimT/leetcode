//! 飞地的数量

use std::collections::VecDeque;

pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    fn bfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) {
        if grid[x][y] != 1 { return; }
        let m = grid.len();
        let n = grid[0].len();
        let mut q = VecDeque::new();
        q.push_back((x, y));
        grid[x][y] = 2;
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] == 1 {
                        q.push_back((nx, ny));
                        grid[nx][ny] = 2;
                    }
                }
            }
        }
    }
    for i in 0..m {
        if grid[i][0] == 1 {
            bfs(&mut grid, i, 0);
        }
        if grid[i][n - 1] == 1 {
            bfs(&mut grid, i, n - 1);
        }
    }
    for j in 1..n - 1 {
        if grid[0][j] == 1 {
            bfs(&mut grid, 0, j);
        }
        if grid[m - 1][j] == 1 {
            bfs(&mut grid, m - 1, j);
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(num_enclaves(vec![vec![0, 0, 0, 0], vec![1, 0, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]), 3);
    assert_eq!(num_enclaves(vec![vec![0, 1, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 0]]), 0);
}
