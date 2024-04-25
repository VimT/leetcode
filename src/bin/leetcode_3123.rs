//! 最短路径中的边

use std::collections::{BinaryHeap, VecDeque};

pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    let mut result = vec![false; edges.len()];
    for (i, edge) in edges.iter().enumerate() {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        g[u].push((v, w, i));
        g[v].push((u, w, i));
    }
    let mut q = BinaryHeap::new();
    q.push((0, 0));
    let mut dis = vec![i32::MAX; n];
    dis[0] = 0;
    while let Some((d, u)) = q.pop() {
        if -d > dis[u] { continue; }
        for &(v, w, _) in &g[u] {
            let n = dis[u] + w;
            if n < dis[v] {
                dis[v] = n;
                q.push((-n, v));
            }
        }
    }
    let mut q = VecDeque::new();
    q.push_back((n - 1, dis[n - 1]));
    while let Some((u, cur)) = q.pop_back() {
        for &(v, w, i) in &g[u] {
            if cur - w == dis[v] {
                result[i] = true;
                q.push_back((v, cur - w));
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool>) {
        assert_eq!(func(6, vec![vec![0, 1, 4], vec![0, 2, 1], vec![1, 3, 2], vec![1, 4, 3], vec![1, 5, 1], vec![2, 3, 1], vec![3, 5, 3], vec![4, 5, 2]]), vec![true, true, true, false, true, true, true, false]);
        assert_eq!(func(4, vec![vec![2, 0, 1], vec![0, 1, 1], vec![0, 3, 4], vec![3, 2, 2]]), vec![true, false, false, true]);
    }
    test(find_answer);
}
