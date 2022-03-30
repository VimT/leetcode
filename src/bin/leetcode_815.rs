//! 公交路线

use std::collections::{HashMap, VecDeque};

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target { return 0; }
    let len = routes.len();
    let mut zhan_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, route) in routes.iter().enumerate() {
        for zhan in route {
            zhan_map.entry(*zhan).or_default().push(i);
        }
    }
    if !zhan_map.contains_key(&source) || !zhan_map.contains_key(&target) { return -1; }
    let mut edge = vec![vec![false; len]; len];
    for (_, routes) in &zhan_map {
        for (i, &xian1) in routes.iter().enumerate() {
            for &xian2 in &routes[i + 1..] {
                edge[xian1][xian2] = true;
                edge[xian2][xian1] = true;
            }
        }
    }
    let mut q = VecDeque::new();
    let mut dis = vec![-1; len];
    for &start in &zhan_map[&source] {
        dis[start] = 1;
        q.push_back(start);
    }
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        for nxt in 0..len {
            if edge[node][nxt] && dis[nxt] == -1 {
                dis[nxt] = dis[node] + 1;
                q.push_back(nxt);
            }
        }
    }
    let mut result = i32::MAX;
    for &end in &zhan_map[&target] {
        if dis[end] != -1 {
            result = result.min(dis[end]);
        }
    }
    if result > 1000 { -1 } else { result }
}

fn main() {
    assert_eq!(num_buses_to_destination(vec![vec![2], vec![2, 8]], 8, 2), 1);
    assert_eq!(num_buses_to_destination(vec![vec![7, 12], vec![4, 5, 15], vec![6], vec![15, 19], vec![9, 12, 13]], 15, 12), -1);
    assert_eq!(num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6), 2);
}
