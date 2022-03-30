//! 颜色交替的最短路径

use std::collections::VecDeque;

pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut rg = vec![vec![]; n];
    let mut bg = vec![vec![]; n];
    for edge in red_edges {
        rg[edge[0] as usize].push(edge[1] as usize);
    }
    for edge in blue_edges {
        bg[edge[0] as usize].push(edge[1] as usize);
    }
    let mut q = VecDeque::new();
    q.push_back((true, 0, 0));
    q.push_back((false, 0, 0));
    let mut blue_result = vec![i32::MAX; n];
    let mut red_result = vec![i32::MAX; n];
    blue_result[0] = 0;
    red_result[0] = 0;
    while !q.is_empty() {
        let (is_red, node, step) = q.pop_front().unwrap();
        let opp_result = if is_red { &mut blue_result } else { &mut red_result };
        let nxts = if is_red { &bg[node] } else { &rg[node] };
        for &next in nxts {
            if step + 1 < opp_result[next] {
                opp_result[next] = step + 1;
                q.push_back((!is_red, next, step + 1));
            }
        }
    }
    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = blue_result[i].min(red_result[i]);
        if result[i] == i32::MAX {
            result[i] = -1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![1, 2], vec![2, 3], vec![3, 1]]), vec![0, 1, 2, 3, 7]);
        assert_eq!(func(3, vec![vec![0, 1], vec![1, 2]], vec![]), vec![0, 1, -1]);
        assert_eq!(func(3, vec![vec![0, 1]], vec![vec![2, 1]]), vec![0, 1, -1]);
    }
    test(shortest_alternating_paths);
}
