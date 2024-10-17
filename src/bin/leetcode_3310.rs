//! 移除可疑的方法

use leetcode::union_find::UnionFind;
use std::collections::VecDeque;

pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut indegree = vec![0; n];
    let mut g = vec![vec![]; n];
    let mut us = UnionFind::new(n);
    for inv in invocations {
        let (a, b) = (inv[0] as usize, inv[1] as usize);
        indegree[b] += 1;
        g[a].push(b);
        us.union(a, b);
    }
    let mut zero = VecDeque::new();
    zero.push_back(k as usize);
    let mut result = vec![0; n];
    let mut num = 1;
    result[k as usize] = 1;
    while let Some(u) = zero.pop_front() {
        for &v in &g[u] {
            if result[v] == 0 {
                zero.push_back(v);
                result[v] = 1;
                num += 1;
            }
        }
    }
    if num < us.size(k as usize) {
        return (0..n as i32).collect();
    }

    (0..n as i32).filter(|&i| result[i as usize] == 0).collect()
}

pub fn remaining_methods2(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let k = k as usize;
    let mut g = vec![vec![]; n];
    for inv in &invocations {
        g[inv[0] as usize].push(inv[1] as usize);
    }
    // 收集所有可疑方法
    let mut suspicious = vec![false; n];
    fn dfs(g: &Vec<Vec<usize>>, u: usize, suspicious: &mut Vec<bool>) {
        suspicious[u] = true;
        for &v in &g[u] {
            if !suspicious[v] {
                dfs(g, v, suspicious);
            }
        }
    }
    dfs(&g, k, &mut suspicious);
    // 检查是否有 【非可疑方法】 -> 【可疑方法】 的边
    for inv in invocations {
        if !suspicious[inv[0] as usize] && suspicious[inv[1] as usize] {
            return (0..n as i32).collect();
        }
    }
    (0..n as i32).filter(|&x| !suspicious[x as usize]).collect()
}

fn main() {
    fn test(func: fn(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(4, 2, vec![vec![1, 0], vec![3, 0], vec![2, 3], vec![2, 0]]), vec![0, 1, 2, 3]);
        assert_eq!(func(3, 2, vec![vec![1, 0], vec![2, 0]]), vec![0, 1, 2]);
        assert_eq!(func(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]), vec![0, 1, 2, 3]);
        assert_eq!(func(5, 0, vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]]), vec![3, 4]);
        assert_eq!(func(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]]), vec![]);
    }
    test(remaining_methods);
    test(remaining_methods2);
}
