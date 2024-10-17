//! 构造符合图结构的二维矩阵

use std::collections::BinaryHeap;

/// 选两个角，用这两个角去判断每个数字的位置
pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in edges {
        let (a, b) = (e[0] as usize, e[1] as usize);
        g[a].push(b);
        g[b].push(a);
    }
    let mut degree_cnt = vec![vec![]; 5];
    for i in 0..n {
        degree_cnt[g[i].len()].push(i);
    }

    fn distance(g: &Vec<Vec<usize>>, u: usize) -> Vec<i32> {
        let mut q = BinaryHeap::new();
        q.push((0, u));
        let mut dis = vec![i32::MAX / 2; g.len()];
        dis[u] = 0;
        while let Some((_, u)) = q.pop() {
            for &v in &g[u] {
                if dis[v] > dis[u] + 1 {
                    dis[v] = dis[u] + 1;
                    q.push((-dis[v], v));
                }
            }
        }
        dis
    }

    if !degree_cnt[1].is_empty() {
        let mut result = vec![vec![0; n]; 1];
        let dis = distance(&g, degree_cnt[1][0]);
        for i in 0..n {
            result[0][dis[i] as usize] = i as i32;
        }
        return result;
    }

    let jiao = &degree_cnt[2];
    let lt = jiao[0];
    let dis_lt = distance(&g, lt);

    let mut other_dis: Vec<(i32, usize)> = (1..jiao.len()).map(|i| (dis_lt[jiao[i]], jiao[i])).collect();
    other_dis.sort_unstable();
    let (m, n) = (other_dis[0].0 as usize + 1, other_dis[1].0 as usize + 1);
    let rt = other_dis[1].1;

    let dis_rt = distance(&g, rt);

    let mut result = vec![vec![0; n]; m];
    for i in 0..m * n {
        let (a, b) = (dis_lt[i], dis_rt[i]);
        let x = (a + b + 1 - n as i32) / 2;
        let y = a - x;
        result[x as usize][y as usize] = i as i32;
    }

    result
}

pub fn construct_grid_layout2(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    // 每种度选一个点
    let mut deg_to_node = vec![n; 5];
    for (i, e) in g.iter().enumerate() {
        deg_to_node[e.len()] = i;
    }
    let row = if deg_to_node[1] != n {
        // 一列
        vec![deg_to_node[1] as i32]
    } else if deg_to_node[4] == n {
        // 两列
        let x = deg_to_node[2];
        let y = g[x].iter().filter(|&&y| g[y].len() == 2).next().copied().unwrap();
        vec![x as i32, y as i32]
    } else {
        // 至少三列
        // 找度数为 2333332 的序列作为第一排
        let mut x = deg_to_node[2];
        let mut row = vec![x as i32];
        let mut pre = x;
        x = g[x][0];
        while g[x].len() == 3 {
            row.push(x as i32);
            for &y in &g[x] {
                if y != pre && g[y].len() < 4 {
                    pre = x;
                    x = y;
                    break;
                }
            }
        }
        row.push(x as i32);
        row
    };

    let mut result = vec![vec![]; n / row.len()];
    let mut vis = vec![false; n];
    for &x in &row {
        vis[x as usize] = true;
    }
    result[0] = row;
    for i in 1..result.len() {
        for x in result[i - 1].clone() {
            for &y in &g[x as usize] {
                if !vis[y] {
                    vis[y] = true;
                    result[i].push(y as i32);
                    break;
                }
            }
        }
    }
    result
}
fn main() {
    use leetcode::vvec;
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        fn verify(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>>, n: i32, edges: Vec<Vec<i32>>) {
            let result = func(n, edges.clone());
            let mut result_edges = Vec::with_capacity(edges.len());
            let mut push = |a: i32, b: i32| {
                result_edges.push(if a > b { vec![b, a] } else { vec![a, b] })
            };
            let m = result.len();
            let n = result[0].len();
            for i in 0..m {
                for j in 0..n {
                    if j + 1 < n { push(result[i][j], result[i][j + 1]); }
                    if i + 1 < m { push(result[i][j], result[i + 1][j]) };
                }
            }
            result_edges.sort_unstable();
            assert_eq!(edges, result_edges, "{:?}", result);
        }
        verify(func, 6, vvec![[0,1],[0,2],[0,4],[1,3],[2,5],[3,4],[4,5]]);
        verify(func, 9, vvec![[0,1],[0,4],[0,5],[1,7],[2,3],[2,4],[2,5],[3,6],[4,6],[4,7],[6,8],[7,8]]);
        verify(func, 5, vvec![[0,1],[1,3],[2,3],[2,4]]);
        verify(func, 4, vvec![[0,1],[0,2],[1,3],[2,3]]);
    }
    test(construct_grid_layout);
    test(construct_grid_layout2);
}
