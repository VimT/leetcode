//! 到达角落需要移除障碍物的最小数目


use std::collections::{BinaryHeap, VecDeque};

pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut heap = BinaryHeap::new();
    heap.push((0, 0, 0));
    let mut vis = vec![vec![false; n]; m];
    vis[0][0] = true;
    while !heap.is_empty() {
        let (cur, x, y) = heap.pop().unwrap();
        if x == m - 1 && y == n - 1 {
            return -cur;
        }
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if !vis[nx][ny] {
                    vis[nx][ny] = true;
                    heap.push((cur - grid[nx][ny], nx, ny));
                }
            }
        }
    }
    unreachable!()
}

/// 答案解法，不用堆，就用简单队列，如果是0就往前面插，如果是1就往后面插
pub fn minimum_obstacles2(grid: Vec<Vec<i32>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut dis = vec![vec![i32::MAX; n]; m];
    dis[0][0] = 0;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if dis[nx][ny] > dis[x][y] + grid[nx][ny] {
                    dis[nx][ny] = dis[x][y] + grid[nx][ny];
                    if grid[nx][ny] == 1 {
                        q.push_back((nx, ny));
                    } else {
                        q.push_front((nx, ny));
                    }
                }
            }
        }
    }
    dis[m - 1][n - 1]
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]), 2);
        assert_eq!(func(vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0]]), 0);
    }
    test(minimum_obstacles);
    test(minimum_obstacles2);
}
