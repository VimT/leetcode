//! 使网格图至少有一条有效路径的最小代价

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

static DIR: [(i32, i32, i32); 4] = [(4, -1, 0), (1, 0, 1), (3, 1, 0), (2, 0, -1)];

/// 优先队列
pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0, 0)));
    let mut cost = vec![vec![i32::MAX; n]; m];
    cost[0][0] = 0;
    while !heap.is_empty() {
        let Reverse((has_cost, x, y)) = heap.pop().unwrap();
        for (d, dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                let next_cost = has_cost + (grid[x][y] != d) as i32;
                if next_cost < cost[nx][ny] {
                    cost[nx][ny] = next_cost;
                    heap.push(Reverse((next_cost, nx, ny)));
                }
            }
        }
    }
    cost[m - 1][n - 1]
}

/// 使用队列，可以在符合方向时push_front优先消费
pub fn min_cost2(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut cost = vec![vec![i32::MAX; n]; m];
    cost[0][0] = 0;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (d, dx, dy) in DIR {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if nx < m && ny < n {
                let not_shun = grid[x][y] != d;
                let next_cost = cost[x][y] + not_shun as i32;
                if next_cost < cost[nx][ny] {
                    cost[nx][ny] = next_cost;
                    if not_shun {
                        q.push_back((nx, ny));
                    } else {
                        q.push_front((nx, ny));
                    }
                }
            }
        }
    }
    cost[m - 1][n - 1]
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]), 0);
        assert_eq!(func(vec![vec![1, 1, 1, 1], vec![2, 2, 2, 2], vec![1, 1, 1, 1], vec![2, 2, 2, 2]]), 3);
        assert_eq!(func(vec![vec![1, 2], vec![4, 3]]), 1);
        assert_eq!(func(vec![vec![2, 2, 2], vec![2, 2, 2]]), 3);
        assert_eq!(func(vec![vec![4]]), 0);
    }
    test(min_cost);
    test(min_cost2);
}
