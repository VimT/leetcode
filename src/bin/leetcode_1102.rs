//! 得分最高的路径


use std::collections::BinaryHeap;

struct UnionSet {
    f: Vec<usize>,
    size: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut xx = self.find(x);
        let mut yy = self.find(y);
        if xx == yy { return; }
        if self.size[xx] < self.size[yy] {
            std::mem::swap(&mut xx, &mut yy);
        }
        self.f[yy] = xx;
        self.size[xx] += self.size[yy];
    }
}


/// 并查集，值从大到小依次添加，如果头和尾能连起来，则是结果
pub fn maximum_minimum_path(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut heap = BinaryHeap::with_capacity(m * n * 2);
    for i in 0..m {
        for j in 0..n {
            if i > 0 {
                heap.push((grid[i][j].min(grid[i - 1][j]), (i - 1) * n + j, i * n + j));
            }
            if j > 0 {
                heap.push((grid[i][j].min(grid[i][j - 1]), i * n + j - 1, i * n + j));
            }
        }
    }
    let mut us = UnionSet::new(m * n);
    loop {
        let (val, x, y) = heap.pop().unwrap();
        us.union(x, y);
        if us.find(0) == us.find(m * n - 1) {
            return val;
        }
    }
}


static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];


/// 二分查找
pub fn maximum_minimum_path2(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut left = i32::MAX;
    let mut right = i32::MIN;
    for line in &grid {
        for &val in line {
            left = left.min(val);
            right = right.max(val + 1);
        }
    }
    fn dfs(grid: &Vec<Vec<i32>>, i: i32, j: i32, visited: &mut Vec<Vec<bool>>, min: i32) -> bool {
        if i == grid.len() as i32 - 1 && j == grid[0].len() as i32 - 1 {
            return true;
        }
        for (dx, dy) in DIR {
            let (nx, ny) = (i + dx, j + dy);
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                if grid[nx as usize][ny as usize] >= min && !visited[nx as usize][ny as usize] {
                    visited[nx as usize][ny as usize] = true;
                    if dfs(grid, nx, ny, visited, min) { return true; }
                }
            }
        }
        false
    }
    while left < right {
        let mid = (left + right) / 2;
        let mut vis = vec![vec![false; n]; m];
        vis[0][0] = true;
        if grid[0][0] >= mid && dfs(&grid, 0, 0, &mut vis, mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}

/// 堆+bfs
pub fn maximum_minimum_path3(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut vis = vec![vec![false; n]; m];
    let mut heap = BinaryHeap::new();
    heap.push((grid[0][0], 0, 0));
    let mut result = grid[0][0].min(grid[m - 1][n - 1]);
    while !heap.is_empty() {
        let (val, x, y) = heap.pop().unwrap();
        result = result.min(val);
        if x == m - 1 && y == n - 1 { return result; }
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if !vis[nx][ny] {
                    vis[nx][ny] = true;
                    heap.push((grid[nx][ny], nx, ny));
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]]), 4);
        assert_eq!(func(vec![vec![2, 2, 1, 2, 2, 2], vec![1, 2, 2, 2, 1, 2]]), 2);
        assert_eq!(func(vec![vec![3, 4, 6, 3, 4], vec![0, 2, 1, 1, 7], vec![8, 8, 3, 2, 7], vec![3, 2, 4, 9, 8], vec![4, 1, 2, 0, 0], vec![4, 6, 5, 4, 3]]), 3);
    }
    test(maximum_minimum_path);
    test(maximum_minimum_path2);
    test(maximum_minimum_path3);
}

