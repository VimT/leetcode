//! 在树上执行操作以后得到的最大分数

pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
    let len = values.len();
    let mut g = vec![vec![]; len];
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        g[a].push(b);
        g[b].push(a);
    }
    // 返回 (有选1个最大分数, 全不选的最大分数)
    fn dfs(d @ (g, values): (&Vec<Vec<usize>>, &Vec<i32>), u: usize, fa: usize) -> (i64, i64) {
        let val = values[u] as i64;
        if g[u].len() == 1 && g[u][0] == fa { return (0, val); }
        let mut a = 0;
        let mut b = 0;
        for &v in &g[u] {
            if v != fa {
                let (aa, bb) = dfs(d, v, u);
                b += bb;
                a += aa;
            }
        }
        (b.max(a + val), b + val)
    }
    dfs((&g, &values), 0, len).0
}

fn main() {
    fn test(func: fn(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64) {
        assert_eq!(func(vec![vec![0, 1]], vec![1, 2]), 2);
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![4, 5]], vec![5, 2, 5, 2, 1, 1]), 11);
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]], vec![20, 10, 9, 7, 4, 3, 5]), 40);
    }
    test(maximum_score_after_operations);
}
