//! 有向无环图中一个节点的所有祖先


use std::collections::VecDeque;

pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut eg = vec![vec![]; n];
    let mut result = vec![vec![]; n];
    for edge in edges {
        eg[edge[0] as usize].push(edge[1] as usize);
    }
    for i in 0..n {
        let mut vis = vec![false; n];
        let mut q = VecDeque::new();
        q.push_back(i);
        vis[i] = true;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for &v in &eg[u] {
                if !vis[v] {
                    vis[v] = true;
                    q.push_back(v);
                    result[v].push(i as i32);
                }
            }
        }
    }
    for i in &mut result {
        i.sort_unstable();
    }
    result
}

fn main() {
    assert_eq!(get_ancestors(8, vec![vec![0, 3], vec![0, 4], vec![1, 3], vec![2, 4], vec![2, 7], vec![3, 5], vec![3, 6], vec![3, 7], vec![4, 6]]), vec![vec![], vec![], vec![], vec![0, 1], vec![0, 2], vec![0, 1, 3], vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3]]);
    assert_eq!(get_ancestors(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]), vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]]);
}
