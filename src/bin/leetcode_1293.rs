//! 网格中的最短路径

use std::collections::VecDeque;

pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0, k));
    let mut seen = vec![vec![-1; n]; m];
    seen[0][0] = k;
    while !q.is_empty() {
        let (x, y, step, k) = q.pop_front().unwrap();
        if x == m - 1 && y == n - 1 {
            return step;
        }
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if ((grid[nx][ny] == 1 && k > 0) || grid[nx][ny] == 0) && k - grid[nx][ny] > seen[nx][ny] {
                    q.push_back((nx, ny, step + 1, k - grid[nx][ny]));
                    seen[nx][ny] = k - grid[nx][ny];
                }
            }
        }
    }
    -1
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 0]], 1), 6);
        assert_eq!(func(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1), -1);
    }
    test(shortest_path);
}
