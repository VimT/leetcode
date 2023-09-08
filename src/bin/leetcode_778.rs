//! 水位上升的泳池中游泳


use std::cmp::Reverse;
use leetcode::union_find::UnionFind;

// timeout
pub fn swim_in_water_dfs(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>, x: usize, y: usize, mut cur: i32, ans: &mut i32) {
        let n = grid.len();
        if cur > *ans { return; }
        if grid[x][y] > cur {
            cur = grid[x][y];
        }
        if x == n - 1 && y == n - 1 {
            if cur < *ans {
                *ans = cur;
            }
        }
        seen[x][y] = true;
        if x > 0 && !seen[x - 1][y] { dfs(grid, seen, x - 1, y, cur, ans); }
        if y > 0 && !seen[x][y - 1] { dfs(grid, seen, x, y - 1, cur, ans); }
        if x < n - 1 && !seen[x + 1][y] { dfs(grid, seen, x + 1, y, cur, ans); }
        if y < n - 1 && !seen[x][y + 1] { dfs(grid, seen, x, y + 1, cur, ans); }
        seen[x][y] = false;
    }
    let n = grid.len();
    let mut ans = i32::max_value();
    let mut seen = vec![vec![false; n]; n];
    dfs(&grid, &mut seen, 0, 0, 0, &mut ans);
    ans
}


pub fn swim_in_water_binary_dfs(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>, x: usize, y: usize, mid: i32) -> bool {
        let n = grid.len();
        if grid[x][y] > mid { return false; }
        if x == n - 1 && y == n - 1 {
            return true;
        }
        seen[x][y] = true;
        if x > 0 && !seen[x - 1][y] && dfs(grid, seen, x - 1, y, mid) { return true; }
        if y > 0 && !seen[x][y - 1] && dfs(grid, seen, x, y - 1, mid) { return true; }
        if x < n - 1 && !seen[x + 1][y] && dfs(grid, seen, x + 1, y, mid) { return true; }
        if y < n - 1 && !seen[x][y + 1] && dfs(grid, seen, x, y + 1, mid) { return true; }
        return false;
    }

    let n = grid.len();
    let mut left = 0_i32;
    let mut right = (n * n) as i32;
    while left < right {
        let mid = left + (right - left) / 2;
        if dfs(&grid, &mut vec![vec![false; n]; n], 0, 0, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

pub fn swim_in_water_union_find(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut edges = Vec::with_capacity(n * n * 2);
    for i in 0..n {
        for j in 0..n {
            let id = i * n + j;
            if i > 0 {
                edges.push((id - n, id, grid[i][j].max(grid[i - 1][j])));
            }
            if j > 0 {
                edges.push((id - 1, id, grid[i][j].max(grid[i][j - 1])));
            }
        }
    }
    edges.sort_unstable_by(|x, y| x.2.cmp(&y.2));
    let mut uf = UnionFind::new(edges.len());
    for edge in edges {
        us.union(edge.0, edge.1);
        if us.find(0) == us.find(n * n - 1) {
            return edge.2;
        }
    }
    return 0;
}

pub fn swim_in_water_dijkstra(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut heap = std::collections::BinaryHeap::new();
    let mut dist = vec![vec![i32::MAX; n]; n];
    dist[0][0] = grid[0][0];
    heap.push((Reverse(grid[0][0]), 0, 0));
    let dirs = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    while !heap.is_empty() {
        let (Reverse(d), x, y) = heap.pop().unwrap();
        if x == y && x == n - 1 {
            break;
        }
        for &(dx, dy) in &dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= n as isize || ny < 0 || ny >= n as isize { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if d.max(grid[nx][ny]) < dist[nx][ny] {
                dist[nx][ny] = d.max(grid[nx][ny]);
                heap.push((Reverse(dist[nx][ny]), nx, ny));
            }
        }
    }
    dist[n - 1][n - 1]
}

struct Heap(Vec<Vec<i32>>, std::collections::BinaryHeap<(i32, [u8; 2])>, u8);

impl Heap {
    fn new(mut grid: Vec<Vec<i32>>, l: u8) -> Self {
        let mut h = std::collections::BinaryHeap::new();
        h.push((-grid[0][0], [0, 0]));
        grid[0][0] = -1;
        Self(grid, h, l)
    }
    fn push(&mut self, v: [u8; 2]) {
        if v[0] < self.2 && v[1] < self.2 && self.0[v[0] as usize][v[1] as usize] >= 0 {
            self.1.push((-self.0[v[0] as usize][v[1] as usize], v));
            self.0[v[0] as usize][v[1] as usize] = -1;
        }
    }
}

// others, best
pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let l = grid.len() as u8 - 1;
    let mut t = Heap::new(grid, l + 1);
    let mut ans = 0;
    while let Some((deep, [v0, v1])) = t.1.pop() {
        ans = ans.min(deep);
        if v0 == v1 && v0 == l { break; }
        (&[[v0, v1 + 1], [v0, v1 - 1], [v0 + 1, v1], [v0 - 1, v1]]).iter().for_each(|&x| t.push(x));
    }
    -ans
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 2], vec![1, 3]]), 3);
        assert_eq!(func(vec![vec![0, 1, 2, 3, 4], vec![24, 23, 22, 21, 5], vec![12, 13, 14, 15, 16], vec![11, 17, 18, 19, 20], vec![10, 9, 8, 7, 6]]), 16);
    }
    // test(swim_in_water);
    test(swim_in_water_binary_dfs);
    test(swim_in_water_dfs);
    test(swim_in_water_dijkstra);
    test(swim_in_water_union_find);
}
