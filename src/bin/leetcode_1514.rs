//! 概率最大的路径

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
struct F64(f64);

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for (edge, prob) in edges.into_iter().zip(succ_prob) {
        g[edge[0] as usize].push((edge[1] as usize, prob));
        g[edge[1] as usize].push((edge[0] as usize, prob));
    }
    let mut q = BinaryHeap::new();
    q.push((F64(1.), start as usize));
    let end = end as usize;
    let mut dis = vec![0.0; n];
    dis[start as usize] = 1.;
    while !q.is_empty() {
        let (_, u) = q.pop().unwrap();
        for &(v, p) in &g[u] {
            if dis[v] < dis[u] * p {
                dis[v] = dis[u] * p;
                q.push((F64(dis[v]), v));
            }
        }
    }
    dis[end]
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64) {
        assert_eq!(func(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![0.5, 0.5, 0.2], 0, 2), 0.25000);
        assert_eq!(func(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![0.5, 0.5, 0.3], 0, 2), 0.30000);
        assert_eq!(func(3, vec![vec![0, 1]], vec![0.5], 0, 2), 0.00000);
    }
    test(max_probability);
}
