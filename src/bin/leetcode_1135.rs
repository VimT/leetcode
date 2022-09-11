//! 最低成本联通所有城市


use std::collections::BinaryHeap;
use std::mem::swap;

struct UnionSet {
    f: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x { x } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut fx = self.find(x);
        let mut fy = self.find(y);
        if fx == fy { return; }
        if self.rank[fx] < self.rank[fy] {
            swap(&mut fx, &mut fy)
        }
        self.rank[fx] += self.rank[fy];
        self.f[fy] = fx;
    }
}

/// 最小生成树，Kruskal算法
pub fn minimum_cost(n: i32, mut connections: Vec<Vec<i32>>) -> i32 {
    let mut us = UnionSet::new(n as usize);
    connections.sort_unstable_by_key(|x| x[2]);
    let mut result = 0;
    for conn in connections {
        if us.find(conn[0] as usize - 1) != us.find(conn[1] as usize - 1) {
            us.union(conn[0] as usize - 1, conn[1] as usize - 1);
            result += conn[2];
        }
    }
    let root = us.find(0);
    if us.rank[root] != n as usize {
        return -1;
    }
    result
}

/// 最小生成树，Prim算法
pub fn minimum_cost2(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for conn in connections {
        let (a, b, w) = (conn[0] as usize - 1, conn[1] as usize - 1, conn[2]);
        g[a].push((b, w));
        g[b].push((a, w));
    }
    let mut heap = BinaryHeap::new();
    let mut vis = vec![false; n];
    heap.push((0, 0));
    while !heap.is_empty() {
        let (dis, cur) = heap.pop().unwrap();
        if vis[cur] { continue; }
        vis[cur] = true;
        result -= dis;
        for &(v, w) in &g[cur] {
            if !vis[v] {
                heap.push((-w, v));
            }
        }
    }
    if vis.iter().all(|x| *x) {
        return result;
    }
    -1
}

fn main() {
    fn test(func: fn(n: i32, connections: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, vec![vec![1, 2, 5], vec![1, 3, 6], vec![2, 3, 1]]), 6);
        assert_eq!(func(4, vec![vec![1, 2, 3], vec![3, 4, 4]]), -1);
    }
    test(minimum_cost);
    test(minimum_cost2);
}
