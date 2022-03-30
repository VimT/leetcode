//! 细分图中的可到达结点

use std::collections::{BinaryHeap, HashMap};

pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
    let n = n as usize;
    let mut ed = vec![vec![]; n];
    for edge in &edges {
        let (u, v, w) = (edge[0], edge[1], edge[2]);
        ed[u as usize].push((v as usize, w));
        ed[v as usize].push((u as usize, w));
    }
    let mut iter = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut dist = vec![i32::MAX; n];

    heap.push((0, 0));
    let mut result = 0;
    while !heap.is_empty() {
        let (step, node) = heap.pop().unwrap();
        let step = -step;
        if step > dist[node] { continue; }
        dist[node] = step;
        // each node will be only iter once
        result += 1;
        for &(nxt, nxt_weight) in &ed[node] {
            iter.insert((node, nxt), (max_moves - step).min(nxt_weight));
            let d2 = step + nxt_weight + 1;
            if d2 <= max_moves && d2 < dist[nxt] {
                dist[nxt] = d2;
                heap.push((-d2, nxt));
            }
        }
    }
    for edge in edges {
        let (u, v, w) = (edge[0], edge[1], edge[2]);
        result += w.min(*iter.get(&(u as usize, v as usize)).unwrap_or(&0) + *iter.get(&(v as usize, u as usize)).unwrap_or(&0));
    }
    result
}

fn main() {
    assert_eq!(reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3), 13);
    assert_eq!(reachable_nodes(vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]], 10, 4), 23);
    assert_eq!(reachable_nodes(vec![vec![1, 2, 4], vec![1, 4, 5], vec![1, 3, 1], vec![2, 3, 4], vec![3, 4, 5]], 17, 5), 1);
}
