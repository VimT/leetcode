//! 网格图中鱼的最大数目

use std::collections::VecDeque;

pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();

    let mut result = 0;
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] > 0 {
                let mut this = grid[i][j];
                q.push_back((i, j));
                grid[i][j] = 0;
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for (dx, dy) in DIR {
                        let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                        if nx < m && ny < n && grid[nx][ny] > 0 {
                            this += grid[nx][ny];
                            grid[nx][ny] = 0;
                            q.push_back((nx, ny));
                        }
                    }
                }
                result = result.max(this);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 2, 1, 0], vec![4, 0, 0, 3], vec![1, 0, 0, 4], vec![0, 3, 2, 0]]), 7);
        assert_eq!(func(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 1]]), 1);
    }
    test(find_max_fish);
}
