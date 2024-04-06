//! 统计树中的合法路径数目


use std::sync::OnceLock;
use leetcode::algorithm::cal_is_prime;
use leetcode::union_find::UnionFind;

/// 遍历质数节点，对每个质数节点的 非质数子树大小做乘法原理
pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    static IS_PRIME: OnceLock<Vec<bool>> = OnceLock::new();
    let prime = IS_PRIME.get_or_init(|| cal_is_prime(1e5 as usize));
    let n = n as usize;
    let mut uf = UnionFind::new(n + 1);
    let mut g = vec![vec![]; n + 1];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        g[u].push(v);
        g[v].push(u);
        if !prime[u] && !prime[v] {
            uf.union(u, v);
        }
    }
    let mut result = 0;
    for i in 1..=n {
        if prime[i] {
            let mut c = 1;
            for &v in &g[i] {
                if !prime[v] {
                    let vr = uf.find(v);
                    let sm = uf.size[vr];
                    result += c * sm;
                    c += sm;
                }
            }
        }
    }
    result as i64
}

/// 思路一样，但dfs 不使用并查集。（实测没有并查集速度快）
pub fn count_paths2(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    static IS_PRIME: OnceLock<Vec<bool>> = OnceLock::new();
    let prime = IS_PRIME.get_or_init(|| cal_is_prime(1e5 as usize));
    let n = n as usize;
    let mut g = vec![vec![]; n + 1];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        g[u].push(v);
        g[v].push(u);
    }
    fn dfs(g: &Vec<Vec<usize>>, prime: &Vec<bool>, u: usize, fa: usize, nodes: &mut Vec<usize>) {
        nodes.push(u);
        for &v in &g[u] {
            if v != fa && !prime[v] {
                dfs(g, prime, v, u, nodes);
            }
        }
    }
    let mut size = vec![0; n + 1];  // 每个连通块的大小
    let mut nodes = vec![];
    let mut result = 0;
    for i in 1..=n {
        if prime[i] {
            let mut s = 0;
            for &u in &g[i] {
                if !prime[u] {
                    if size[u] == 0 {
                        nodes.clear();
                        dfs(&g, prime, u, u, &mut nodes);
                        for &v in &nodes {
                            size[v] = nodes.len() as i64;
                        }
                    }
                    result += s * size[u];
                    s += size[u];
                }
            }
            result += s;  // 从质数点i出发的路径数
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(5, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]]), 4);
        assert_eq!(func(5, vec![vec![1, 5], vec![2, 1], vec![4, 5], vec![3, 2]]), 4);
        assert_eq!(func(6, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]]), 6);
    }
    test(count_paths);
    test(count_paths2);
}
