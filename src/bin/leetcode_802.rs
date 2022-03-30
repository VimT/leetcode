//! 找到最终的安全状态

use std::collections::VecDeque;

pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let len = graph.len();
    let mut seen = vec![false; len];
    let mut out_degree = vec![0; len];
    let mut m = vec![vec![]; len];
    for from in 0..len {
        for &to in &graph[from] {
            m[to as usize].push(from);
            out_degree[from as usize] += 1;
        }
    }
    let mut q = VecDeque::new();
    for i in 0..len {
        if out_degree[i] == 0 {
            q.push_back(i);
        }
    }
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        if seen[node] { continue; }
        seen[node] = true;
        for &from in &m[node] {
            out_degree[from] -= 1;
            if out_degree[from] == 0 {
                q.push_back(from);
            }
        }
    }
    (0..len as i32).filter(|x| seen[*x as usize]).collect()
}

fn main() {
    assert_eq!(eventual_safe_nodes(vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![]]), vec![2, 4, 5, 6]);
    assert_eq!(eventual_safe_nodes(vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]]), vec![4]);
}
