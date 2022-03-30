//! 树中距离之和

pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    fn dfs(sz: &mut Vec<i32>, dp: &mut Vec<i32>, graph: &Vec<Vec<usize>>, u: usize, f: usize) {
        sz[u] = 1;
        dp[u] = 0;
        for &v in &graph[u].clone() {
            if v == f {
                continue;
            }
            dfs(sz, dp, graph, v, u);
            dp[u] += dp[v] + sz[v];
            sz[u] += sz[v];
        }
    }

    fn dfs2(sz: &mut Vec<i32>, dp: &mut Vec<i32>, graph: &Vec<Vec<usize>>, ans: &mut Vec<i32>, u: usize, f: usize) {
        ans[u] = dp[u];
        for &v in &graph[u] {
            if v == f {
                continue;
            }
            let pu = dp[u];
            let pv = dp[v];
            let su = sz[u];
            let sv = sz[v];

            dp[u] -= dp[v] + sz[v];
            sz[u] -= sz[v];
            dp[v] += dp[u] + sz[u];
            sz[v] += sz[u];

            dfs2(sz, dp, graph, ans, v, u);

            dp[u] = pu;
            dp[v] = pv;
            sz[u] = su;
            sz[v] = sv;
        }
    }

    let n = n as usize;
    let mut ans = vec![0; n];
    let mut sz = vec![0; n];
    let mut dp = vec![0; n];
    let mut graph = vec![vec![]; n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }
    dfs(&mut sz, &mut dp, &graph, 0, 0);
    dfs2(&mut sz, &mut dp, &graph, &mut ans, 0, 0);

    ans
}


fn main() {
    assert_eq!(sum_of_distances_in_tree(6, vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]), vec![8, 12, 6, 10, 10, 10]);
    assert_eq!(sum_of_distances_in_tree(1, vec![]), vec![0]);
    assert_eq!(sum_of_distances_in_tree(2, vec![vec![1, 0]]), vec![1, 1]);
}
