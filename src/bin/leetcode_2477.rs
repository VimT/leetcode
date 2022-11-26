//! 到达首都的最少油耗


use std::collections::VecDeque;

/// dfs
pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let n = roads.len() + 1;
    let mut g = vec![vec![]; n];
    for road in roads {
        g[road[0] as usize].push(road[1] as usize);
        g[road[1] as usize].push(road[0] as usize);
    }

    // return 子树大小
    fn dfs(g: &Vec<Vec<usize>>, u: usize, fa: usize, seats: i64, result: &mut i64) -> i64 {
        let mut tree_size = 1;
        for &v in &g[u] {
            if v != fa {
                let sub_tree_size = dfs(g, v, u, seats, result);
                let mut car = sub_tree_size / seats;
                if sub_tree_size % seats != 0 { car += 1; }
                *result += car;
                tree_size += sub_tree_size;
            }
        }
        tree_size
    }

    let mut result = 0;
    dfs(&g, 0, 0, seats as i64, &mut result);
    result
}

/// 拓扑排序
pub fn minimum_fuel_cost2(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let n = roads.len() + 1;
    let mut g = vec![vec![]; n];
    let mut people = vec![1; n]; // 每个节点当前有多少人

    let mut degree = vec![0; n];
    for road in roads {
        let s = road[0] as usize;
        let t = road[1] as usize;
        degree[s] += 1;
        degree[t] += 1;
        g[s].push(t);
        g[t].push(s);
    }
    let mut q = VecDeque::new();
    for i in 1..n {
        if degree[i] == 1 {
            q.push_back(i);
        }
    }

    let mut vis = vec![false; n];
    let seats = seats as i64;

    let mut result = 0;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        vis[u] = true;

        for &v in &g[u] {
            if !vis[v] {
                let mut car = people[u] / seats;
                if car * seats < people[u] {
                    car += 1;
                }
                result += car;
                people[v] += people[u];

                if v == 0 { // 到达首都
                    break;
                }

                degree[u] -= 1;
                degree[v] -= 1;
                if degree[v] == 1 {
                    q.push_back(v);
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(roads: Vec<Vec<i32>>, seats: i32) -> i64) {
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5), 3);
        assert_eq!(func(vec![vec![3, 1], vec![3, 2], vec![1, 0], vec![0, 4], vec![0, 5], vec![4, 6]], 2), 7);
        assert_eq!(func(vec![], 1), 0);
    }
    test(minimum_fuel_cost);
    test(minimum_fuel_cost2);
}
