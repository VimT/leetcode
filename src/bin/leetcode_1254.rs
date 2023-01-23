//! 统计封闭岛屿的数目

use std::collections::VecDeque;

pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    let mut result = 0;
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if grid[i][j] == 0 {
                q.push_back((i, j));
                let mut ok = true;
                grid[i][j] = 2;
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for (dx, dy) in DIR {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if grid[nx][ny] == 0 {
                                q.push_back((nx, ny));
                                grid[nx][ny] = 2;
                            }
                        } else {
                            ok = false;
                        }
                    }
                }
                if ok {
                    result += 1;
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1, 1, 1, 1, 1, 1, 0], vec![1, 0, 0, 0, 0, 1, 1, 0], vec![1, 0, 1, 0, 1, 1, 1, 0], vec![1, 0, 0, 0, 0, 1, 0, 1], vec![1, 1, 1, 1, 1, 1, 1, 0]]), 2);
        assert_eq!(func(vec![vec![0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 1, 1, 1, 0]]), 1);
        assert_eq!(func(vec![vec![1, 1, 1, 1, 1, 1, 1],
                             vec![1, 0, 0, 0, 0, 0, 1],
                             vec![1, 0, 1, 1, 1, 0, 1],
                             vec![1, 0, 1, 0, 1, 0, 1],
                             vec![1, 0, 1, 1, 1, 0, 1],
                             vec![1, 0, 0, 0, 0, 0, 1],
                             vec![1, 1, 1, 1, 1, 1, 1]], ), 2);
    }
    test(closed_island);
}
