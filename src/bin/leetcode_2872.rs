//! 可以被 K 整除连通块的最大数目

pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let k = k as i64;
    struct DFS<'a> {
        g: &'a Vec<Vec<usize>>,
        values: Vec<i32>,
        k: i64,
        result: i32,
    }
    impl<'a> DFS<'a> {
        fn dfs(&mut self, u: usize, fa: usize) -> i64 {
            let mut sum = 0;
            for &v in &self.g[u] {
                if v != fa {
                    sum += self.dfs(v, u);
                }
            }
            sum += self.values[u] as i64;
            if sum % self.k == 0 {
                self.result += 1;
            }
            sum
        }
    }
    let mut dfs = DFS { g: &g, values, k, result: 0 };
    dfs.dfs(0, 0);
    dfs.result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(5, vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]], vec![1, 8, 1, 4, 4], 6), 2);
        assert_eq!(func(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]], vec![3, 0, 6, 1, 5, 2, 1], 3), 3);
    }
    test(max_k_divisible_components);
}
