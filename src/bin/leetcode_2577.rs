//! 在网格图中访问一个格子的最少时间

use std::cmp::Reverse;
use std::collections::BinaryHeap;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][1] > 1 && grid[1][0] > 1 {
        return -1;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, 0, 0)));
    let mut min_time = vec![vec![i32::MAX; n]; m];
    while !q.is_empty() {
        let Reverse((cur_time, x, y)) = q.pop().unwrap();
        if x == m - 1 && y == n - 1 {
            return cur_time;
        }
        for (dx, dy) in DIR {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if nx < m && ny < n {
                let next_time = if grid[nx][ny] <= cur_time + 1 {
                    cur_time + 1
                } else {
                    grid[nx][ny] + ((grid[nx][ny] - cur_time - 1) & 1)
                };
                if next_time < min_time[nx][ny] {
                    min_time[nx][ny] = next_time;
                    q.push(Reverse((next_time, nx, ny)));
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]), 7);
        assert_eq!(func(vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]]), -1);
    }
    test(minimum_time);
}
