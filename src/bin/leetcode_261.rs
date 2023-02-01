//! 以图判树

use std::collections::VecDeque;
use leetcode::union_set::UnionSet;

/// 判断图是不是树，1.是连通图 2.不存在环
pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    let mut edge_len = edges.len() as i32;
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut vis = vec![false; n];
    fn dfs(i: usize, vis: &mut Vec<bool>, g: &Vec<Vec<usize>>, edge_len: &mut i32) {
        vis[i] = true;
        for &j in &g[i] {
            if !vis[j] {
                *edge_len -= 1;
                dfs(j, vis, g, edge_len);
            }
        }
    }
    dfs(0, &mut vis, &g, &mut edge_len);
    vis.iter().all(|x| *x) && edge_len == 0
}

pub fn valid_tree_bfs(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    let mut edge_len = edges.len() as i32;
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut vis = vec![false; n];
    vis[0] = true;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for &v in &g[u] {
            if !vis[v] {
                q.push_back(v);
                edge_len -= 1;
                vis[v] = true;
            }
        }
    }
    vis.iter().all(|x| *x) && edge_len == 0
}


/// 并查集
pub fn valid_tree_us(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let mut us = UnionSet::new(n as usize);
    for edge in edges {
        let tmp1 = us.find(edge[0] as usize);
        let tmp2 = us.find(edge[1] as usize);
        if tmp1 == tmp2 {
            return false;
        }
        us.union(edge[0] as usize, edge[1] as usize);
    }
    let root = us.find(0);
    for i in 1..n as usize {
        if us.find(i) != root { return false; }
    }
    true
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]), true);
        assert_eq!(func(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]), false);
    }
    test(valid_tree);
    test(valid_tree_bfs);
    test(valid_tree_us);
}
