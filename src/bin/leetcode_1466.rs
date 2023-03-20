//! 重新规划路线

use std::collections::VecDeque;
use leetcode::graph::Graph;

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = Graph::new(n, n - 1);
    for conn in connections {
        g.add(conn[0] as usize, conn[1] as usize, 1);
        g.add(conn[1] as usize, conn[0] as usize, 0);
    }
    let mut q = VecDeque::with_capacity(n);
    q.push_back((0, 0));
    let mut result = 0;
    while !q.is_empty() {
        let (u, fa) = q.pop_front().unwrap();
        for (v, &cost) in g.neigh(u) {
            if v != fa {
                result += cost;
                q.push_back((v, u));
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, connections: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(6, vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]), 3);
        assert_eq!(func(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]), 2);
        assert_eq!(func(3, vec![vec![1, 0], vec![2, 0]]), 0);
    }
    test(min_reorder);
}
