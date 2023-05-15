//! 统计完全连通分量的数量

use std::collections::VecDeque;

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![false; n]; n];
    for edge in edges {
        g[edge[0] as usize][edge[1] as usize] = true;
        g[edge[1] as usize][edge[0] as usize] = true;
    }
    let mut vis = vec![false; n];
    let mut q = VecDeque::new();
    let mut result = 0;
    for i in 0..n {
        if !vis[i] {
            q.push_back(i);
            vis[i] = true;
            let mut edges = 0;
            let mut nodes = 1;
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                for v in 0..n {
                    if g[u][v] {
                        edges += 1;
                        if !vis[v] {
                            vis[v] = true;
                            q.push_back(v);
                            nodes += 1;
                        }
                    }
                }
            }
            if edges == (nodes * (nodes - 1)) {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]), 3);
        assert_eq!(func(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]), 1);
    }
    test(count_complete_components);
}
