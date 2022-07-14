//! 不同岛屿的数量 II

use std::collections::HashSet;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

pub fn num_distinct_islands2(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &Vec<Vec<i32>>, road: &mut Vec<(i32, i32)>, x: usize, y: usize, seen: &mut Vec<Vec<bool>>) {
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] == 1 && !seen[nx][ny] {
                    seen[nx][ny] = true;
                    road.push((nx as i32, ny as i32));
                    dfs(grid, road, nx, ny, seen);
                }
            }
        }
    }
    fn canonical(road: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        if road.is_empty() { return road; }
        let mut result = vec![];
        let len = road.len();
        let mut out = vec![(0, 0); len];
        let mut xs = vec![0; len];
        let mut ys = vec![0; len];
        // 8个方向依次遍历
        for c in 0..8 {
            let mut t = 0;
            for &(x, y) in &road {
                xs[t] = match c {
                    0 | 1 => x,
                    2 | 3 => -x,
                    4 | 5 => y,
                    _ => -y,
                };
                ys[t] = if c <= 3 {
                    if c % 2 == 0 { y } else { -y }
                } else {
                    if c % 2 == 0 { x } else { -x }
                };
                t += 1;
            }
            let mx = *xs.iter().min().unwrap();
            let my = *ys.iter().min().unwrap();
            for j in 0..len {
                out[j] = ((xs[j] - mx), (ys[j] - my));
            }
            out.sort_unstable();
            if out > result {
                result = out.clone();
            }
        }
        result
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
                road_set.insert(canonical(road));
            }
        }
    }
    road_set.len() as i32
}


fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1, 0, 0, 1], vec![1, 0, 0, 1, 1], vec![1, 1, 0, 1, 0], vec![1, 0, 0, 1, 1]]), 2);
        assert_eq!(func(vec![vec![0, 1]]), 1);
        assert_eq!(func(vec![vec![1, 1, 0, 0, 0], vec![1, 0, 0, 0, 0], vec![0, 0, 0, 0, 1], vec![0, 0, 0, 1, 1]]), 1);
        assert_eq!(func(vec![vec![1, 1, 0, 0, 0], vec![1, 1, 0, 0, 0], vec![0, 0, 0, 1, 1], vec![0, 0, 0, 1, 1]]), 1);
    }
    test(num_distinct_islands2);
}
