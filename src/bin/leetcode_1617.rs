//! 统计子树中城市之间最大距离

/// 二进制枚举+求二叉树的最大直径
pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize - 1].push(edge[1] as usize - 1);
        g[edge[1] as usize - 1].push(edge[0] as usize - 1);
    }
    let mut result = vec![0; n - 1];
    fn dfs(g: &Vec<Vec<usize>>, nodes: usize, u: usize, fa: usize, cnt: &mut u32, max_len: &mut i32) -> i32 {
        *cnt += 1;
        let mut max_h = 0;
        for &v in &g[u] {
            if v != fa && nodes >> v & 1 == 1 {
                let h = dfs(g, nodes, v, u, cnt, max_len);
                *max_len = (*max_len).max(h + max_h);
                max_h = max_h.max(h);
            }
        }
        max_h + 1
    }
    for i in 3..1usize << n {
        let size = i.count_ones();
        if size < 2 { continue; }
        for j in 0..n {
            if i >> j & 1 == 1 {
                let mut cnt = 0;
                let mut max_len = 0;
                dfs(&g, i, j, n, &mut cnt, &mut max_len);
                if cnt == size {
                    result[max_len as usize - 1] += 1;
                }
                break;
            }
        }
    }
    result
}

/// 暴力枚举 i 和 j 作为直径的两个端点 ，那么从 i 到 j 的这条简单路径是直径，这上面的每个点都必须选。
/// 这条路上还有哪些点是可以选的，哪些点是不能选的？（具体看灵神题解图）
pub fn count_subgraphs_for_each_diameter2(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize - 1].push(edge[1] as usize - 1);
        g[edge[1] as usize - 1].push(edge[0] as usize - 1);
    }
    let mut result = vec![0; n - 1];
    let mut dis = vec![vec![0; n]; n];
    fn dfs(g: &Vec<Vec<usize>>, start: usize, u: usize, fa: usize, dis: &mut Vec<Vec<i32>>) {
        for &v in &g[u] {
            if v != fa {
                dis[start][v] = dis[start][u] + 1;
                dfs(g, start, v, u, dis);
            }
        }
    }
    for i in 0..n {
        dfs(&g, i, i, n, &mut dis);
    }
    fn dfs2(g: &Vec<Vec<usize>>, u: usize, fa: usize, dis: &Vec<Vec<i32>>, i: usize, j: usize, d: i32) -> i32 {
        let mut cnt = 1;  // 选u
        for &v in &g[u] {
            if v != fa {
                if (dis[i][v] < d || dis[i][v] == d && v > j) && (dis[j][v] < d || dis[j][v] == d && v > i) {
                    cnt *= dfs2(g, v, u, dis, i, j, d);  // 这个子树的可选数，乘法原理
                }
            }
        }
        if dis[i][u] + dis[j][u] > d {
            cnt += 1; // 不选u
        }
        cnt
    }
    for i in 0..n {
        for j in i + 1..n {
            let d = dis[i][j];
            result[d as usize - 1] += dfs2(&g, i, n, &dis, i, j, d);
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(7, vec![vec![1, 4], vec![1, 3], vec![2, 5], vec![2, 6], vec![3, 6], vec![6, 7]]), vec![6, 7, 7, 5, 2, 0]);
        assert_eq!(func(4, vec![vec![1, 2], vec![2, 3], vec![2, 4]]), vec![3, 4, 0]);
        assert_eq!(func(2, vec![vec![1, 2]]), vec![1]);
        assert_eq!(func(3, vec![vec![1, 2], vec![2, 3]]), vec![2, 1]);
    }
    test(count_subgraphs_for_each_diameter);
    test(count_subgraphs_for_each_diameter2);
}
