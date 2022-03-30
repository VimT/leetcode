//! 岛屿的最大面积

use std::collections::VecDeque;

const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut vis = vec![vec![false; n]; m];
    let mut q = VecDeque::new();
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 && !vis[i][j] {
                q.push_back((i, j));
                let mut area = 1;
                vis[i][j] = true;
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for (dx, dy) in DIR {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if grid[nx][ny] == 1 && !vis[nx][ny] {
                                vis[nx][ny] = true;
                                area += 1;
                                q.push_back((nx, ny));
                            }
                        }
                    }
                }
                result = result.max(area);
            }
        }
    }
    result
}

fn main() {
    assert_eq!(max_area_of_island(vec![vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0], vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0], vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]]), 6);
    assert_eq!(max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]), 0);
}
