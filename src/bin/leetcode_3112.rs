//! 访问消失节点的最少时间


use std::collections::BinaryHeap;

pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        let (u, v, t) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
        g[u].push((v, t));
        g[v].push((u, t));
    }
    let mut dist = vec![i64::MAX; n];
    dist[0] = 0;
    let mut q = BinaryHeap::new();
    q.push((0, 0));
    while let Some((ndis, u)) = q.pop() {
        if -ndis > dist[u] { continue; }
        for &(v, t) in &g[u] {
            let new_dis = dist[u] + t;
            if new_dis < disappear[v] as i64 && dist[v] > new_dis {
                dist[v] = new_dis;
                q.push((-dist[v], v));
            }
        }
    }
    dist.into_iter().map(|x| x as i32).collect()
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(3, vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]], vec![1, 1, 5]), vec![0, -1, 4]);
        assert_eq!(func(3, vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]], vec![1, 3, 5]), vec![0, 2, 3]);
        assert_eq!(func(2, vec![vec![0, 1, 1]], vec![1, 1]), vec![0, -1]);
    }
    test(minimum_time);
}
