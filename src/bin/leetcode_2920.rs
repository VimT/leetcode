//! 收集所有金币可获得的最大积分

pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
    let len = coins.len();
    let mut g = vec![vec![]; len];
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        g[a].push(b);
        g[b].push(a);
    }
    struct DFS<'a> {
        g: &'a Vec<Vec<usize>>,
        coins: Vec<i32>,
        k: i32,
        cache: Vec<Vec<i32>>,
    }
    impl<'a> DFS<'a> {
        // 在节点u，已经右移j次的情况下，最多能收集到的金币数
        fn dfs(&mut self, u: usize, fa: usize, j: usize) -> i32 {
            if self.cache[u][j] != -1 { return self.cache[u][j]; }
            let mut a = (self.coins[u] >> j) - self.k;
            let mut b = self.coins[u] >> j + 1;
            for &v in &self.g[u] {
                if v != fa {
                    a += self.dfs(v, u, j);
                    if j < 13 {
                        b += self.dfs(v, u, j + 1)
                    }
                }
            }
            self.cache[u][j] = a.max(b);
            self.cache[u][j]
        }
    }
    let mut dfs = DFS { g: &g, coins, k, cache: vec![vec![-1; 14]; len] };
    dfs.dfs(0, len, 0)
}

pub fn maximum_points2(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
    let mut e = vec![vec![]; coins.len()];
    for v in edges {
        let (a, b) = (v[0] as usize, v[1] as usize);
        e[a].push(b);
        e[b].push(a);
    }
    // 这个部分是 Rust 中的模式匹配语法。d @ &(e, c, k) 完成了几件事情
    fn dfs(d @ (e, c, k): (&Vec<Vec<usize>>, &Vec<i32>, i32), n: usize, p: usize, result: &mut [i32; 15]) {
        let mut r = [0; 15];
        for &m in &e[n] {
            if m == p { continue; }
            dfs(d, m, n, &mut r);
        }
        let mut x = c[n];
        for i in 0..14 {
            result[i] += (x - k + r[i]).max(x >> 1 + r[i + 1]);
            x >>= 1;
        }
    }
    let mut result = [0; 15];
    dfs((&e, &coins, k), 0, usize::MAX, &mut result);
    result[0]
}

fn main() {
    fn test(func: fn(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![10, 10, 3, 3], 5), 11);
        assert_eq!(func(vec![vec![0, 1], vec![0, 2]], vec![8, 4, 4], 0), 16);
    }
    test(maximum_points);
    test(maximum_points2);
}
