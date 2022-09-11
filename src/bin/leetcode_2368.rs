//! 受限条件下可到达节点的数目

use std::collections::{HashSet, VecDeque};

pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let res: HashSet<i32> = restricted.into_iter().collect();
    let mut g = vec![vec![]; n as usize];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut seen = vec![false; n as usize];
    seen[0] = true;
    let mut result = 1;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for &v in &g[u] {
            if !seen[v] && !res.contains(&(v as i32)) {
                result += 1;
                seen[v] = true;
                q.push_back(v);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32) {
        assert_eq!(func(7, vec![vec![0, 1], vec![1, 2], vec![3, 1], vec![4, 0], vec![0, 5], vec![5, 6]], vec![4, 5]), 4);
        assert_eq!(func(7, vec![vec![0, 1], vec![0, 2], vec![0, 5], vec![0, 4], vec![3, 2], vec![6, 5]], vec![4, 2, 1]), 3);
    }
    test(reachable_nodes);
}
