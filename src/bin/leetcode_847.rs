//! 访问所有节点的最短路径

use std::collections::VecDeque;

/// 广搜：（节点，已经遍历了哪些节点，已经走了多少步）。通过 seen[ 节点][mask] 减枝
pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let len = graph.len();
    let mut q = VecDeque::new();
    let mut seen = vec![vec![false; 1 << len]; len];
    for i in 0..len {
        q.push_back((i, 1 << i, 0));
        seen[i][1 << i] = true;
    }
    let final_ = (1 << len) - 1;
    while !q.is_empty() {
        let (node, mask, dist) = q.pop_front().unwrap();
        if mask == final_ {
            return dist;
        } else {
            for &nxt in &graph[node] {
                let nxt_mask = mask | (1 << nxt);
                if !seen[nxt as usize][nxt_mask] {
                    seen[nxt as usize][nxt_mask] = true;
                    q.push_back((nxt as usize, nxt_mask, dist + 1));
                }
            }
        }
    }
    0
}

fn main() {
    assert_eq!(shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]), 4);
    assert_eq!(shortest_path_length(vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]]), 4);
}
