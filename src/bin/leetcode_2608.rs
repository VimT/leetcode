//! 图中的最短环


use std::collections::VecDeque;

/// 设 u 和 v 之间有一条边长为 w 的边，dis(u,v) 表示删除 u 和 v 之间的连边之后，u 和 v 之间的最短路。
/// 那么无向图中的最小环是 dis(u,v)+w。
pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in &edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut result = i32::MAX;
    let mut q = VecDeque::with_capacity(n);
    let mut dis = vec![i32::MAX; n];
    for edge in &edges {
        q.clear();
        dis.fill(i32::MAX);
        let s = edge[0] as usize;
        let t = edge[1] as usize;
        dis[s] = 0;
        for &v in &g[s] {
            if v != t {
                q.push_back((2, v));
            }
        }
        'out: while !q.is_empty() {
            let (d, u) = q.pop_front().unwrap();
            for &v in &g[u] {
                if dis[v] > d + 1 {
                    dis[v] = d + 1;
                    if v == t { break 'out; }
                    q.push_back((d + 1, v));
                }
            }
        }
        result = result.min(dis[t]);
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![0, 7], vec![0, 6], vec![5, 7], vec![5, 6]]), 4);
        assert_eq!(func(4, vec![vec![1, 2], vec![0, 1], vec![3, 2], vec![1, 3]]), 3);
        assert_eq!(func(7, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 3]]), 3);
        assert_eq!(func(4, vec![vec![0, 1], vec![0, 2]]), -1);
    }
    test(find_shortest_cycle);
}
