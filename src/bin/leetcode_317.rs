//! 离建筑物最近的距离

use std::collections::VecDeque;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// bfs + 一些省空间和时间的优化
pub fn shortest_distance(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    let mut empty_place = 0;
    let mut dist = grid.clone();
    // 对每个建筑，每轮更新sum
    let mut sum = grid.clone();
    let mut result = i32::MAX;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                result = i32::MAX;
                q.push_back((i, j));
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for (dx, dy) in DIR {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if grid[nx][ny] == empty_place {
                                // 用empty_place标记，也能充当vis的作用
                                grid[nx][ny] -= 1;
                                dist[nx][ny] = dist[x][y] + 1;
                                sum[nx][ny] += dist[x][y];
                                q.push_back((nx, ny));
                                result = result.min(sum[nx][ny]);
                            }
                        }
                    }
                }
                // 每次都要更新空格标记，没更新到就是访问不到
                empty_place -= 1;
            }
        }
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 1], vec![0, 1, 1, 0, 0, 1], vec![1, 0, 0, 1, 0, 1], vec![1, 0, 1, 0, 0, 1], vec![1, 0, 0, 0, 0, 1], vec![0, 1, 1, 1, 1, 0]]), 88);
        assert_eq!(func(vec![vec![1, 0, 2, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0]]), 7);
        assert_eq!(func(vec![vec![1, 0]]), 1);
        assert_eq!(func(vec![vec![1]]), -1);
    }
    test(shortest_distance);
}
