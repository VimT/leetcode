//! 迷宫中离入口最近的出口

use std::collections::VecDeque;

pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let m = maze.len();
    let n = maze[0].len();
    let mut q = VecDeque::new();
    q.push_back((entrance[0], entrance[1], 0));
    let mut vis = vec![vec![false; n]; m];
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while !q.is_empty() {
        let (x, y, cur) = q.pop_front().unwrap();
        if vis[x as usize][y as usize] {
            continue;
        }
        vis[x as usize][y as usize] = true;
        if (x != entrance[0] || y != entrance[1]) && (x == 0 || x == m as i32 - 1 || y == 0 || y == n as i32 - 1) {
            return cur;
        }
        for &(dx, dy) in &dir {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && maze[nx as usize][ny as usize] == '.' {
                q.push_back((nx, ny, cur + 1));
            }
        }
    }
    -1
}

fn main() {
    assert_eq!(nearest_exit(vec![vec!['+', '+', '.', '+'], vec!['.', '.', '.', '+'], vec!['+', '+', '+', '.']], vec![1, 2]), 1);
    assert_eq!(nearest_exit(vec![vec!['+', '+', '+'], vec!['.', '.', '.'], vec!['+', '+', '+']], vec![1, 0]), 2);
    assert_eq!(nearest_exit(vec![vec!['.', '+']], vec![0, 0]), -1);
}
