//! 不同岛屿的数量

use std::collections::HashSet;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

pub fn num_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &Vec<Vec<i32>>, road: &mut Vec<(i32, i32)>, x: usize, y: usize, seen: &mut Vec<Vec<bool>>) {
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] == 1 && !seen[nx][ny] {
                    seen[nx][ny] = true;
                    road.push((dx, dy));
                    dfs(grid, road, nx, ny, seen);
                    road.push((-dx, -dy));
                }
            }
        }
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut road_set = HashSet::new();
    let mut seen = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 && !seen[i][j] {
                let mut road = vec![];
                dfs(&grid, &mut road, i, j, &mut seen);
                road_set.insert(road);
            }
        }
    }
    road_set.len() as i32
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0], vec![1, 1, 1], vec![0, 1, 0]]), 2);
        assert_eq!(func(vec![vec![1, 1, 0, 0, 0], vec![1, 1, 0, 0, 0], vec![0, 0, 0, 1, 1], vec![0, 0, 0, 1, 1]]), 1);
        assert_eq!(func(vec![vec![1, 1, 0, 1, 1], vec![1, 0, 0, 0, 0], vec![0, 0, 0, 0, 1], vec![1, 1, 0, 1, 1]]), 3);
    }
    test(num_distinct_islands);
}
