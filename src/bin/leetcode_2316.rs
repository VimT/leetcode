//! 统计无向图中无法互相到达点对数


use std::collections::VecDeque;

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut seen = vec![false; n];
    let mut q = VecDeque::new();
    let mut result = 0;
    for i in 0..n {
        if !seen[i] {
            seen[i] = true;
            let mut size = 1;
            q.push_back(i);
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                for &v in &g[u] {
                    if !seen[v] {
                        seen[v] = true;
                        size += 1;
                        q.push_back(v);
                    }
                }
            }
            result += (n - size) * size;
        }
    }
    (result / 2) as i64
}

fn main() {
    assert_eq!(count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 0);
    assert_eq!(count_pairs(7, vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]), 14);
}
