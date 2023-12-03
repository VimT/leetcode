//! 可以到达每一个节点的最少边反转次数

pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        g[u].push((v, 0));
        g[v].push((u, 1));
    }
    fn dfs(g: &Vec<Vec<(usize, i32)>>, u: usize, fa: usize) -> i32 {
        let mut result = 0;
        for &(v, w) in &g[u] {
            if v != fa {
                result += dfs(g, v, u) + w;
            }
        }
        result
    }
    fn dfs2(g: &Vec<Vec<(usize, i32)>>, u: usize, fa: usize, cur: i32, result: &mut Vec<i32>) {
        result[u] = cur;
        for &(v, w) in &g[u] {
            if v != fa {
                dfs2(g, v, u, cur + if w == 0 { 1 } else { -1 }, result);
            }
        }
    }
    let a = dfs(&g, 0, n);
    let mut result = vec![0; n];
    dfs2(&g, 0, n, a, &mut result);
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(4, vec![vec![2, 0], vec![2, 1], vec![1, 3]]), vec![1, 1, 0, 2]);
        assert_eq!(func(3, vec![vec![1, 2], vec![2, 0]]), vec![2, 0, 1]);
    }
    test(min_edge_reversals);
}
