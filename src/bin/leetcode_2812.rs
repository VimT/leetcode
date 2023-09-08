//! 找出最安全路径

use std::collections::{BinaryHeap, VecDeque};
use leetcode::union_find::UnionFind;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

/// 最大化最小值，二分
/// 从1扩散n步，然后看通不通。
pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    let run = |mut grid: Vec<Vec<i32>>| {
        if grid[0][0] == 1 { return false; }
        let n = grid.len();
        let mut q = VecDeque::new();
        grid[0][0] = 1;
        q.push_back((0, 0));
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            if (x, y) == (n - 1, n - 1) { return true; }
            for (dx, dy) in DIR {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                if nx < n && ny < n && grid[nx][ny] == 0 {
                    grid[nx][ny] = 2;
                    q.push_back((nx, ny));
                }
            }
        }
        false
    };

    let n = grid.len();
    if !run(grid.clone()) { return 0; }
    let mut left = 0;
    let mut right = n;
    let mut thief = vec![];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 { thief.push((i, j)); }
        }
    }
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut q: VecDeque<(usize, usize, usize)> = thief.iter().copied().map(|(x, y)| (x, y, mid - 1)).collect();
        let mut grid = grid.clone();
        while !q.is_empty() {
            let (x, y, c) = q.pop_front().unwrap();
            for (dx, dy) in DIR {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                if nx < n && ny < n && grid[nx][ny] == 0 && c > 0 {
                    grid[nx][ny] = 1;
                    q.push_back((nx, ny, c - 1));
                }
            }
        }
        if run(grid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

/// 从1点扩散n步得到距离图。一种特殊的dijkstra找从 (0,0) 对 (i,j) 的路径最小化最大值
pub fn maximum_safeness_factor2(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut values = vec![vec![0; n]; n];
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                q.push_back((i, j));
                visited[i][j] = true;
            }
        }
    }
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if nx < n && ny < n && grid[nx][ny] == 0 && !visited[nx][ny] {
                values[nx][ny] = values[x][y] + 1;
                visited[nx][ny] = true;
                q.push_back((nx, ny));
            }
        }
    }
    let mut heap = BinaryHeap::new();
    heap.push((values[0][0], 0, 0));
    let mut dist = vec![vec![0; n]; n];
    dist[0][0] = values[0][0];
    while !heap.is_empty() {
        let (safety, x, y) = heap.pop().unwrap();
        if (x, y) == (n - 1, n - 1) {
            return safety;
        }
        for (dx, dy) in DIR {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if nx < n && ny < n && grid[nx][ny] == 0 {
                let new_safety = safety.min(values[nx][ny]);
                if new_safety > dist[nx][ny] {
                    dist[nx][ny] = new_safety;
                    heap.push((new_safety, nx, ny));
                }
            }
        }
    }
    0
}

/// 2的思路再扩展，也可以使用并查集，从步数大的格子遍历，看 find(0) == find(n*n-1)
pub fn maximum_safeness_factor3(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut g = vec![];
    let mut cur = vec![];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                cur.push((i, j));
            }
        }
    }
    while !cur.is_empty() {
        g.push(cur.clone());
        let mut next = vec![];
        for (x, y) in cur {
            for (dx, dy) in DIR {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                if nx < n && ny < n && grid[nx][ny] == 0 {
                    grid[nx][ny] = 1;
                    next.push((nx, ny));
                }
            }
        }
        cur = next;
    }
    for line in &mut grid { line.fill(0); }
    let mut uf = UnionFind::new(n * n);
    while !g.is_empty() {
        for (x, y) in g.pop().unwrap() {
            for (dx, dy) in DIR {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                if nx < n && ny < n && grid[nx][ny] == 1 {
                    us.union(x * n + y, nx * n + ny);
                }
            }
            grid[x][y] = 1;
        }
        if us.find(0) == us.find(n * n - 1) {
            return g.len() as i32;
        }
    }

    0
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0,1,1],vec![0,1,1],vec![0,0,0]]), 1);
        assert_eq!(func(vec![vec![1]]), 0);
        assert_eq!(func(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]), 2);
        assert_eq!(func(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]), 0);
        assert_eq!(func(vec![vec![0, 0, 0, 1], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![1, 0, 0, 0]]), 2);
    }
    test(maximum_safeness_factor);
    test(maximum_safeness_factor2);
    test(maximum_safeness_factor3);
}
