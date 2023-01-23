//! 阈值距离内邻居最少的城市

use std::collections::BinaryHeap;

/// 每个点执行一次 dijkstra
pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        if edge[2] <= distance_threshold {
            g[edge[0] as usize].push((edge[1] as usize, edge[2]));
            g[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }
    }
    let mut min_cnt = i32::MAX;
    let mut result = 0;
    for i in 0..n {
        let mut dis = vec![i32::MAX; n];
        dis[i] = 0;
        let mut q = BinaryHeap::new();
        q.push((0, i));
        let mut cnt = 0;
        while !q.is_empty() {
            let (_, u) = q.pop().unwrap();
            for &(v, w) in &g[u] {
                if dis[v] > dis[u] + w {
                    if dis[u] + w <= distance_threshold {
                        if dis[v] > distance_threshold {
                            cnt += 1;
                        }
                        dis[v] = dis[u] + w;
                        q.push((dis[v], v));
                    }
                }
            }
        }
        if cnt <= min_cnt {
            min_cnt = cnt;
            result = i;
        }
    }
    result as i32
}

/// Floyd全源最短路
pub fn find_the_city2(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n = n as usize;
    let mut dis = vec![vec![i32::MAX >> 1; n]; n];
    for edge in edges {
        if edge[2] <= distance_threshold {
            dis[edge[0] as usize][edge[1] as usize] = edge[2];
            dis[edge[1] as usize][edge[0] as usize] = edge[2];
        }
    }
    for i in 0..n { dis[i][i] = 0; }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
            }
        }
    }
    let mut result = 0;
    let mut min_cnt = i32::MAX;
    for i in 0..n {
        let cnt = dis[i].iter().filter(|x| **x <= distance_threshold).count() as i32;
        if cnt <= min_cnt {
            min_cnt = cnt;
            result = i as i32;
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32) {
        assert_eq!(func(4, vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]], 4), 3);
        assert_eq!(func(5, vec![vec![0, 1, 2], vec![0, 4, 8], vec![1, 2, 3], vec![1, 4, 2], vec![2, 3, 1], vec![3, 4, 1]], 2), 0);
    }
    test(find_the_city);
    test(find_the_city2);
}
