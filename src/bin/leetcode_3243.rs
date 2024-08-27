//! 新增道路查询后的最短距离 I

use std::collections::VecDeque;

pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[i].push(i + 1);
    }
    queries.into_iter().map(|q| {
        let (u, v) = (q[0] as usize, q[1] as usize);
        g[u].push(v);
        let mut q = VecDeque::new();
        q.push_back(0);
        let mut dist = vec![i32::MAX / 2; n];
        dist[0] = 0;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            if u == n - 1 { break; }
            for &v in &g[u] {
                let d = dist[u] + 1;
                if d < dist[v] {
                    dist[v] = d;
                    q.push_back(v);
                }
            }
        }
        dist[n - 1]
    }).collect()
}

fn main() {
    fn test(func: fn(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(5, vec![vec![1, 3], vec![2, 4]]), vec![3, 3]);
        assert_eq!(func(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]), vec![3, 2, 1]);
        assert_eq!(func(4, vec![vec![0, 3], vec![0, 2]]), vec![1, 1]);
    }
    test(shortest_distance_after_queries);
}
