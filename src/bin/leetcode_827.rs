//! 最大人工岛

use std::collections::VecDeque;

pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut land = vec![vec![0; n]; n];
    let mut land_idx = 0;
    let mut q = VecDeque::new();
    let mut land_size = vec![0];
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 && land[i][j] == 0 {
                land_idx += 1;
                land[i][j] = land_idx;
                q.push_back((i, j));
                let mut size = 1;
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for (dx, dy) in DIR {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if grid[nx][ny] == 1 && land[nx][ny] == 0 {
                                land[nx][ny] = land_idx;
                                size += 1;
                                q.push_back((nx, ny));
                            }
                        }
                    }
                }
                land_size.push(size);
            }
        }
    }
    let mut result = *land_size.iter().max().unwrap();
    if result as usize == n * n {
        return result;
    }
    result += 1;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 0 {
                let mut lands = vec![];
                for (dx, dy) in DIR {
                    let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                        let land_idx = land[nx as usize][ny as usize];
                        if !lands.contains(&land_idx) {
                            lands.push(land_idx);
                        }
                    }
                }
                result = result.max(1 + lands.into_iter().map(|x| land_size[x]).sum::<i32>());
            }
        }
    }
    result
}

fn main() {
    assert_eq!(largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
    assert_eq!(largest_island(vec![vec![1, 1], vec![1, 0]]), 4);
    assert_eq!(largest_island(vec![vec![1, 1], vec![1, 1]]), 4);
}
