//! 前往目标的最小代价

use std::collections::{BinaryHeap, HashMap};

/// 离散化，跑Dijkstra
pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
    let mut points: HashMap<(i32, i32), usize> = HashMap::new();
    points.insert((start[0], start[1]), 0);
    points.insert((target[0], target[1]), 0);
    for road in &special_roads {
        points.insert((road[0], road[1]), 0);
        points.insert((road[2], road[3]), 0);
    }
    let points_vec: Vec<(i32, i32)> = points.keys().cloned().collect();
    for (v, i) in points.values_mut().zip(0..) {
        *v = i;
    }
    let len = points.len();
    fn dis(a: (i32, i32), b: (i32, i32)) -> i32 {
        return (a.0 - b.0).abs() + (a.1 - b.1).abs();
    }
    let mut g = vec![vec![0; len]; len];
    for i in 0..len {
        for j in 0..len {
            g[i][j] = dis(points_vec[i], points_vec[j]);
        }
    }
    for road in &special_roads {
        let i = points[&(road[0], road[1])];
        let j = points[&(road[2], road[3])];
        g[i][j] = g[i][j].min(road[4]);
    }
    let mut q = BinaryHeap::new();
    let mut dis = vec![i32::MAX; len];
    let start = points[&(start[0], start[1])];
    let target = points[&(target[0], target[1])];
    dis[start] = 0;
    q.push((0, start));
    while !q.is_empty() {
        let (_, u) = q.pop().unwrap();
        for v in 0..len {
            if dis[v] > dis[u] + g[u][v] {
                dis[v] = dis[u] + g[u][v];
                q.push((-dis[v], v));
            }
        }
    }
    dis[target]
}

/// 直接跑最短路
pub fn minimum_cost2(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
    let len = special_roads.len();
    let src = len;
    let dst = len + 1;
    let mut dis = vec![i32::MAX; len + 2];
    dis[src] = 0;
    let mut vis = vec![false; len + 2];
    let (sx, sy) = (start[0], start[1]);
    let (tx, ty) = (target[0], target[1]);
    loop {
        // 未遍历过的点里最小的那个
        let mut u = dst;
        for (i, &d) in dis.iter().enumerate() {
            if !vis[i] && d < dis[u] {
                u = i;
            }
        }
        if u == dst { return dis[dst]; }
        let (ux, uy) = if u < len { (special_roads[u][2], special_roads[u][3]) } else { (sx, sy) };
        vis[u] = true;
        dis[dst] = dis[dst].min(dis[u] + (ux - tx).abs() + (uy - ty).abs());
        // 松弛
        for (v, road) in special_roads.iter().enumerate() {
            let (x1, y1) = (road[0], road[1]);
            let (vx, vy) = (road[2], road[3]);
            dis[v] = dis[v].min(dis[u] + (ux - vx).abs() + (uy - vy).abs()) // 直接走到v
                .min(dis[u] + (ux - x1).abs() + (uy - y1).abs() + road[4]); // 经过特殊路径走到v
        }
    }
}

fn main() {
    fn test(func: fn(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![1, 1], vec![4, 5], vec![vec![1, 2, 3, 3, 2], vec![3, 4, 4, 5, 1]]), 5);
        assert_eq!(func(vec![3, 2], vec![5, 7], vec![vec![3, 2, 3, 4, 4], vec![3, 3, 5, 5, 5], vec![3, 4, 5, 6, 6]]), 7);
    }
    test(minimum_cost);
    test(minimum_cost2);
}
