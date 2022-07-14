//! 迷宫 II

use std::collections::VecDeque;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
    let m = maze.len();
    let n = maze[0].len();
    let mut q = VecDeque::new();
    let start = (start[0] as usize, start[1] as usize);
    let target = (destination[0] as usize, destination[1] as usize);
    q.push_back((start.0, start.1));
    let mut dis = vec![vec![i32::MAX; n]; m];
    dis[start.0][start.1] = 0;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let mut nx = x as i32;
            let mut ny = y as i32;
            let mut cnt = 0;
            while (nx + dx) >= 0 && (nx + dx) < m as i32 && (ny + dy) >= 0 && (ny + dy) < n as i32 && maze[(nx + dx) as usize][(ny + dy) as usize] == 0 {
                nx += dx;
                ny += dy;
                cnt += 1;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if dis[nx][ny] > dis[x][y] + cnt {
                dis[nx][ny] = dis[x][y] + cnt;
                q.push_back((nx, ny));
            }
        }
    }
    if dis[target.0][target.1] == i32::MAX { -1 } else { dis[target.0][target.1] }
}

fn main() {
    fn test(func: fn(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![1, 1, 0, 1, 1], vec![0, 0, 0, 0, 0]], vec![0, 4], vec![4, 4]), 12);
        assert_eq!(func(vec![vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![1, 1, 0, 1, 1], vec![0, 0, 0, 0, 0]], vec![0, 4], vec![3, 2]), -1);
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0], vec![1, 1, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1], vec![0, 1, 0, 0, 0]], vec![4, 3], vec![0, 1]), -1);
    }
    test(shortest_distance);
}
