//! 收集树中金币

use std::collections::VecDeque;

/// 拓扑排序，消去两层
pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = coins.len();
    let mut g = vec![Vec::with_capacity(3); n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut degree: Vec<i32> = g.iter().map(|x| x.len() as i32).collect();
    let mut left_edge = n as i32 - 1;
    let mut q = VecDeque::new();
    for i in 0..n {
        if degree[i] == 1 && coins[i] == 0 {
            q.push_back(i);
        }
    }
    // 拓扑排序，去掉所有没有金币的叶子节点
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        left_edge -= 1;
        for &v in &g[u] {
            degree[v] -= 1;
            if degree[v] == 1 && coins[v] == 0 {
                q.push_back(v);
            }
        }
    }

    // 去掉2层叶子节点
    for i in 0..n {
        if degree[i] == 1 && coins[i] == 1 {
            q.push_back(i);
        }
    }
    left_edge -= q.len() as i32;
    for u in q {
        for &v in &g[u] {
            degree[v] -= 1;
            if degree[v] == 1 {
                left_edge -= 1;
            }
        }
    }
    left_edge.max(0) * 2
}

fn main() {
    fn test(func: fn(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![0, 0], vec![vec![0, 1]]), 0);
        assert_eq!(func(vec![1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5], vec![4, 6], vec![2, 7], vec![7, 8], vec![3, 9], vec![8, 10], vec![8, 11], vec![6, 12], vec![7, 13], vec![11, 14], vec![10, 15]]), 4);
        assert_eq!(func(vec![0, 1, 1], vec![vec![0, 1], vec![0, 2]]), 0);
        assert_eq!(func(vec![1, 0, 0, 0, 0, 1], vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), 2);
        assert_eq!(func(vec![0, 0, 0, 1, 1, 0, 0, 1], vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![5, 6], vec![5, 7]]), 2);
    }
    test(collect_the_coins);
}
