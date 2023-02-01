//! 最小体力消耗路径


use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use leetcode::union_set::UnionSet;

static DIR: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];


pub fn minimum_effort_path_binary_search(heights: Vec<Vec<i32>>) -> i32 {
    let m = heights.len();
    let n = heights[0].len();
    let mut left = 0;
    let mut right = 1e6 as i32 - 1;
    let mut ans = 0;
    while left <= right {
        let mid = left + (right - left) / 2;
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        let mut seen = vec![false; m * n];
        seen[0] = true;
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            for i in 0..4 {
                let nx = x + DIR[i].0;
                if nx < 0 || nx >= m as isize { continue; }
                let ny = y + DIR[i].1;
                if ny < 0 || ny >= n as isize { continue; }
                let (x, y, nx, ny) = (x as usize, y as usize, nx as usize, ny as usize);
                if !seen[nx * n + ny] && (heights[x][y] - heights[nx][ny]).abs() <= mid {
                    q.push_back((nx as isize, ny as isize));
                    seen[nx * n + ny] = true;
                }
            }
        }
        if seen[m * n - 1] {
            ans = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    ans
}

pub fn minimum_effort_path_union_find(heights: Vec<Vec<i32>>) -> i32 {
    let m = heights.len();
    let n = heights[0].len();
    let mut edges = Vec::with_capacity(m * n * 2);
    for i in 0..m {
        for j in 0..n {
            let id = i * n + j;
            if i > 0 {
                edges.push((id - n, id, (heights[i][j] - heights[i - 1][j]).abs()));
            }
            if j > 0 {
                edges.push((id - 1, id, (heights[i][j] - heights[i][j - 1]).abs()));
            }
        }
    }
    let mut us = UnionSet::new(m * n);
    edges.sort_unstable_by(|x, y| x.2.cmp(&y.2));
    for edge in edges {
        us.union(edge.0, edge.1);
        if us.find(0) == us.find(m * n - 1) {
            return edge.2;
        }
    }
    return 0;
}


pub fn minimum_effort_path_dijkstra(heights: Vec<Vec<i32>>) -> i32 {
    let m = heights.len();
    let n = heights[0].len();
    let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0));
    let mut dist = vec![i32::MAX; m * n];
    dist[0] = 0;
    let mut seen = vec![false; m * n];
    while !heap.is_empty() {
        let (Reverse(d), x, y) = heap.pop().unwrap();
        let id = x * n + y;
        if seen[id] { continue; }
        if x == m - 1 && y == n - 1 {
            break;
        }
        seen[id] = true;
        for i in 0..4 {
            let (x, y) = (x as isize, y as isize);
            let nx = x + DIR[i].0;
            if nx < 0 || nx >= m as isize { continue; }
            let ny = y + DIR[i].1;
            if ny < 0 || ny >= n as isize { continue; }
            let (x, y, nx, ny) = (x as usize, y as usize, nx as usize, ny as usize);
            if (heights[x][y] - heights[nx][ny]).abs().max(d) < dist[nx * n + ny] {
                dist[nx * n + ny] = d.max((heights[x][y] - heights[nx][ny]).abs());
                heap.push((Reverse(dist[nx * n + ny]), nx, ny));
            }
        }
    }
    return dist[m * n - 1];
}

fn main() {
    fn test(func: fn(heights: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]), 2);
        assert_eq!(func(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]), 1);
        assert_eq!(func(vec![vec![1, 2, 1, 1, 1], vec![1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1], vec![1, 1, 1, 2, 1]]), 0);
    }
    test(minimum_effort_path_binary_search);
    test(minimum_effort_path_dijkstra);
    test(minimum_effort_path_union_find);
}
