//! 修改图中的边权

use std::collections::BinaryHeap;

/// 两次dijkstra，第一次 按照全1的路径，第二次按照 target与全1 差值
pub fn modified_graph_edges(n: i32, mut edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for (i, edge) in edges.iter().enumerate() {
        g[edge[0] as usize].push((edge[1] as usize, i));
        g[edge[1] as usize].push((edge[0] as usize, i));
    }
    let source = source as usize;
    let dest = destination as usize;

    let mut q = BinaryHeap::new();
    const INF: i32 = i32::MAX / 2;
    let mut dis = vec![INF; n];
    dis[source] = 0;
    q.push((0, source));
    while !q.is_empty() {
        let (d, u) = q.pop().unwrap();
        if dis[u] < -d { continue; } // 这个可写可不写
        for &(v, i) in &g[u] {
            let mut w = edges[i][2];
            if w == -1 { w = 1; }
            if dis[v] > dis[u] + w {
                dis[v] = dis[u] + w;
                q.push((-dis[v], v));
            }
        }
    }
    if dis[dest] > target { return vec![]; }
    let need = target - dis[dest];

    let mut dis1 = vec![INF; n];
    dis1[source] = 0;
    q.push((0, source));
    while !q.is_empty() {
        let (d, u) = q.pop().unwrap();
        if u == dest { break; }
        if dis1[u] < -d { continue; }
        for &(v, i) in &g[u] {
            let mut w = edges[i][2];
            // 精髓：如果当前边没有决定值
            if w == -1 {
                w = 1; // 默认按照全1遍历
                let z = dis[v] + need - dis1[u];
                if z >= 1 {  // 如果改成z，可以让 dis1[v] 符合target，就改
                    w = z;
                    edges[i][2] = w;
                }
            }
            if dis1[v] > dis1[u] + w {
                dis1[v] = dis1[u] + w;
                q.push((-dis1[v], v));
            }
        }
    }
    if dis1[dest] != target { return vec![]; }

    for edge in &mut edges {
        if edge[2] == -1 {
            edge[2] = 2e9 as i32;
        }
    }
    edges
}

/// 需要找一个未知边，从source到这个边为 dis1，从destination到这个边为dis2，那么这个边的 w = target - dis1 - dis2。
/// 从source, destination 到这个边的最短路，经过的未知边 w = 1，其他没有经过的未知边 w = 2e9
pub fn modified_graph_edges2(n: i32, mut edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
    const INF: i64 = 2_000_000_000;

    let n = n as usize;
    let s = source as usize;
    let t = destination as usize;
    let target = target as i64;

    let mut dis = vec![vec![INF; n]; n];
    for i in 0..n {
        dis[i][i] = 0;
    }

    let mut todo = vec![];
    for (i, edge) in edges.iter().enumerate() {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
        if w == -1 {
            todo.push((i, u, v));
        } else {
            dis[u][v] = w;
            dis[v][u] = w;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
            }
        }
    }

    if dis[s][t] < target {
        return vec![];
    }

    if dis[s][t] == target {
        for (i, _, _) in todo {
            edges[i][2] = INF as i32;
        }
        return edges;
    }

    let mut found = false;
    for (id, u, v) in todo {
        if found {
            edges[id][2] = INF as i32;
            continue;
        }
        let left = dis[s][u] + dis[v][t];
        let right = dis[s][v] + dis[u][t];
        if left.min(right) + 1 <= target {
            edges[id][2] = (target - left.min(right)) as i32;
            found = true;
        } else {
            edges[id][2] = 1;
            for x in 0..n {
                for y in 0..n {
                    dis[x][y] = dis[x][y].min(dis[x][u] + 1 + dis[v][y]).min(dis[x][v] + 1 + dis[u][y]);
                }
            }
        }
    }

    if !found && dis[s][t] > target {
        return vec![];
    }

    return edges;
}

fn main() {
    type Fn = fn(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>>;
    fn test(func: Fn) {
        fn check(func: Fn, n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32, can: bool) {
            let edges = func(n, edges, source, destination, target);
            if can {
                assert_ne!(edges.len(), 0, "shortest path can be target but return empty");
                let mut g = vec![vec![]; n as usize];
                for edge in &edges {
                    g[edge[0] as usize].push((edge[1] as usize, edge[2]));
                    g[edge[1] as usize].push((edge[0] as usize, edge[2]));
                }
                let mut q = BinaryHeap::new();
                let mut dis = vec![i32::MAX / 2; n as usize];
                dis[source as usize] = 0;
                q.push((0, source as usize));
                while !q.is_empty() {
                    let (_, u) = q.pop().unwrap();
                    for &(v, w) in &g[u] {
                        if dis[v] > dis[u] + w {
                            dis[v] = dis[u] + w;
                            q.push((-dis[v], v));
                        }
                    }
                }
                assert_eq!(target, dis[destination as usize], "{:?}", edges);
            } else {
                assert_eq!(edges.len(), 0);
            }
        }

        check(func, 5, vec![vec![1, 2, 1], vec![0, 1, 1], vec![0, 4, -1], vec![4, 3, -1], vec![3, 2, -1]], 2, 0, 2, true);
        check(func, 13, vec![vec![0, 3, 67], vec![1, 0, 82], vec![2, 7, 37], vec![1, 9, 13], vec![5, 1, 84], vec![10, 5, 63], vec![10, 4, 17], vec![4, 11, 65], vec![2, 4, 85], vec![2, 6, 45], vec![6, 8, 70], vec![12, 8, 97], vec![9, 4, 34], vec![12, 9, 48], vec![2, 1, 19], vec![11, 12, 43], vec![8, 4, 62], vec![6, 11, 48], vec![3, 5, 50], vec![8, 7, 86], vec![9, 6, 86], vec![4, 3, 82], vec![4, 5, 91], vec![6, 5, 96], vec![7, 11, 40], vec![2, 12, 87], vec![1, 12, 11], vec![8, 3, 92], vec![0, 9, 71], vec![12, 4, 40], vec![1, 3, 71], vec![6, 12, 100], vec![9, 2, 48], vec![8, 11, 75], vec![6, 10, 91], vec![5, 11, 74], vec![3, 7, 40], vec![8, 9, 44], vec![3, 10, 25], vec![0, 11, -1], vec![0, 5, 21], vec![10, 8, 44], vec![10, 0, 32], vec![10, 9, 52], vec![3, 11, 72], vec![12, 5, 21], vec![7, 4, 46], vec![12, 10, 46], vec![8, 2, -1], vec![0, 4, 21], vec![3, 9, 38], vec![1, 6, 28], vec![4, 1, 77], vec![0, 6, 98], vec![6, 4, 92], vec![10, 2, 97], vec![5, 9, 33], vec![2, 0, 74], vec![1, 10, 88], vec![2, 11, -1], vec![0, 7, 83], vec![12, 7, 70], vec![3, 6, 64], vec![11, 10, 35], vec![12, 3, 54], vec![10, 7, 83], vec![2, 5, 86], vec![1, 7, 18], vec![11, 1, 82], vec![7, 9, 80], vec![2, 3, 15], vec![5, 8, 65], vec![8, 1, 80], vec![12, 0, 30], vec![5, 7, 14]], 0, 8, 48, true);
        check(func, 5, vec![vec![2, 0, -1], vec![4, 1, 1], vec![2, 4, 8], vec![3, 4, 10], vec![2, 1, 1], vec![4, 0, 10], vec![1, 0, 8], vec![1, 3, 9], vec![0, 3, -1]], 2, 3, 10, true);
        check(func, 5, vec![vec![1, 4, 1], vec![2, 4, -1], vec![3, 0, 2], vec![0, 4, -1], vec![1, 3, 10], vec![1, 0, 10]], 0, 2, 15, true);
        check(func, 5, vec![vec![3, 0, -1], vec![2, 1, -1], vec![4, 1, 9], vec![3, 4, -1], vec![4, 0, 6], vec![2, 3, 5], vec![4, 2, 8], vec![3, 1, 7], vec![1, 0, 6], vec![0, 2, 9]], 4, 1, 10, false);
        check(func, 4, vec![vec![0, 1, -1], vec![1, 2, -1], vec![3, 1, -1], vec![3, 0, 2], vec![0, 2, 5]], 2, 3, 8, false);
        check(func, 3, vec![vec![1, 0, 3], vec![1, 2, 4]], 0, 2, 8, false);
        check(func, 5, vec![vec![4, 1, -1], vec![2, 0, -1], vec![0, 3, -1], vec![4, 3, -1]], 0, 1, 5, true);
        check(func, 3, vec![vec![0, 1, -1], vec![0, 2, 5]], 0, 2, 6, false);
        check(func, 4, vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, -1]], 0, 2, 6, true);
    }
    test(modified_graph_edges);
    test(modified_graph_edges2);
}
