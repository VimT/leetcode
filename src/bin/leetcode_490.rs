//! 迷宫

use std::collections::VecDeque;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
    let m = maze.len();
    let n = maze[0].len();
    let mut q = VecDeque::new();
    q.push_back((start[0] as usize, start[1] as usize));
    let target = (destination[0] as usize, destination[1] as usize);
    let mut vis = vec![vec![false; n]; m];
    vis[start[0] as usize][start[1] as usize] = true;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if (x, y) == target {
            return true;
        }
        for (dx, dy) in DIR {
            let mut nx = x as i32;
            let mut ny = y as i32;
            while (nx + dx) >= 0 && (nx + dx) < m as i32 && (ny + dy) >= 0 && (ny + dy) < n as i32 && maze[(nx + dx) as usize][(ny + dy) as usize] == 0 {
                nx += dx;
                ny += dy;
            }
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if !vis[nx][ny] {
                    vis[nx][ny] = true;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool) {
        assert_eq!(func(vec![vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![1, 1, 0, 1, 1], vec![0, 0, 0, 0, 0]], vec![0, 4], vec![4, 4]), true);
        assert_eq!(func(vec![vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![1, 1, 0, 1, 1], vec![0, 0, 0, 0, 0]], vec![0, 4], vec![3, 2]), false);
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0], vec![1, 1, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1], vec![0, 1, 0, 0, 0]], vec![4, 3], vec![0, 1]), false);
    }
    test(has_path);
}
