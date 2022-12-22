//! 两个城市间路径的最小分数


use std::collections::VecDeque;

pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n + 1];
    for road in roads {
        g[road[0] as usize].push((road[1] as usize, road[2]));
        g[road[1] as usize].push((road[0] as usize, road[2]));
    }
    fn dfs(g: &Vec<Vec<(usize, i32)>>, u: usize, seen: &mut Vec<bool>, result: &mut i32) {
        for &(v, score) in &g[u] {
            *result = (*result).min(score);
            if !seen[v] {
                seen[v] = true;
                dfs(g, v, seen, result);
            }
        }
    }

    let mut seen = vec![false; n + 1];
    seen[1] = true;
    let mut result = i32::MAX;
    dfs(&g, 1, &mut seen, &mut result);
    result
}

pub fn min_score_bfs(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n + 1];
    for road in roads {
        g[road[0] as usize].push((road[1] as usize, road[2]));
        g[road[1] as usize].push((road[0] as usize, road[2]));
    }

    let mut q = VecDeque::new();
    q.push_back(1);
    let mut seen = vec![false; n + 1];
    seen[1] = true;
    let mut result = i32::MAX;
    while !q.is_empty() {
        let u = q.pop_back().unwrap();
        for &(v, score) in &g[u] {
            result = result.min(score);
            if !seen[v] {
                seen[v] = true;
                q.push_back(v);
            }
        }
    }
    result
}


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

pub fn min_score_union_set(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut us = UnionSet::new(n);
    for road in &roads {
        us.union(road[0] as usize - 1, road[1] as usize - 1);
    }

    let root = us.find(0);
    let mut result = i32::MAX;
    for road in roads {
        if us.find(road[0] as usize - 1) == root {
            result = result.min(road[2]);
        }
    }

    result
}


fn main() {
    fn test(func: fn(n: i32, roads: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(6, vec![vec![4, 5, 7468], vec![6, 2, 7173], vec![6, 3, 8365], vec![2, 3, 7674], vec![5, 6, 7852], vec![1, 2, 8547], vec![2, 4, 1885], vec![2, 5, 5192], vec![1, 3, 4065], vec![1, 4, 7357]]), 1885);
        assert_eq!(func(4, vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]), 5);
        assert_eq!(func(4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]]), 2);
    }
    test(min_score);
    test(min_score_bfs);
    test(min_score_union_set);
}
