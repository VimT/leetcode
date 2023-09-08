//! 边权重均等查询


/// 树链剖分求LCA
pub fn min_operations_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    // 树链剖分
    struct HLD<'a> {
        g: &'a Vec<Vec<(usize, i32)>>,
        // 父节点
        fa: Vec<usize>,
        // 节点的深度
        depth: Vec<i32>,
        // 子树大小
        size: Vec<i32>,
        // 节点所在链的头结点
        head: Vec<usize>,
        // 重链的子节点
        son: Vec<usize>,
        // dfs次序，可以用来做线段树查询
        dfn: Vec<usize>,
        // 从根节点到每个节点的次数
        info: Vec<[i32; 26]>,
    }

    impl<'a> HLD<'a> {
        fn new(g: &'a Vec<Vec<(usize, i32)>>) -> Self {
            let n = g.len();
            Self {
                g,
                son: vec![0; n],
                fa: vec![0; n],
                depth: vec![0; n],
                size: vec![0; n],
                head: vec![0; n],
                dfn: vec![0; n],
                info: vec![[0; 26]; n],
            }
        }
        fn dfs(&mut self, u: usize, fa: usize, d: i32, cur: &mut [i32; 26]) {
            self.fa[u] = fa;
            self.depth[u] = d;
            self.size[u] = 1;
            self.info[u] = cur.clone();
            for &(v, w) in &self.g[u] {
                if v != fa {
                    cur[w as usize] += 1;
                    self.dfs(v, u, d + 1, cur);
                    cur[w as usize] -= 1;
                    self.size[u] += self.size[v];
                    if self.size[v] > self.size[self.son[u]] {
                        self.son[u] = v;
                    }
                }
            }
        }

        fn decompose(&mut self, u: usize, h: usize, order: &mut usize) {
            self.head[u] = h;
            *order += 1;
            self.dfn[u] = *order;
            if self.son[u] != 0 { self.decompose(self.son[u], h, order); }
            for &(v, _) in &self.g[u] {
                if v != self.fa[u] && v != self.son[u] {
                    self.decompose(v, v, order);
                }
            }
        }
    }

    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in &edges {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2] - 1);
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut hld = HLD::new(&g);
    hld.dfs(0, n, 1, &mut [0; 26]);
    hld.decompose(0, 0, &mut 0);
    let HLD { head, depth, fa: parent, info, .. } = hld;
    queries.into_iter().map(|query| {
        let (mut u, mut v) = (query[0] as usize, query[1] as usize);
        while head[u] != head[v] {
            if depth[head[u]] < depth[head[v]] { std::mem::swap(&mut u, &mut v); }
            u = parent[head[u]];
        }

        if depth[u] > depth[v] { std::mem::swap(&mut u, &mut v); }
        let lca = u;
        let (u, v) = (query[0] as usize, query[1] as usize);
        let (sum, max) = info[u].iter().zip(&info[v]).zip(&info[lca]).map(|((a, b), c)| *a + *b - *c * 2).fold((0, 0), |(sum, max), num| (sum + num, max.max(num)));
        sum - max
    }).collect()
}

/// 倍增求 LCA
pub fn min_operations_queries2(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2] - 1);
        g[u].push((v, w));
        g[v].push((u, w));
    }
    fn dfs(g: &Vec<Vec<(usize, i32)>>, u: usize, fa: usize, d: i32, cur: &mut [i32; 26], depth: &mut Vec<i32>, parent: &mut Vec<Vec<usize>>, info: &mut Vec<[i32; 26]>) {
        depth[u] = d;
        info[u] = cur.clone();
        for &(v, w) in &g[u] {
            if v != fa {
                parent[0][v] = u;
                cur[w as usize] += 1;
                dfs(g, v, u, d + 1, cur, depth, parent, info);
                cur[w as usize] -= 1;
            }
        }
    }
    let mx = 64 - n.leading_zeros() as usize; // 2^14 > 10^4
    let mut parent = vec![vec![n; n]; mx];
    let mut info = vec![[0; 26]; n];
    let mut depth = vec![0; n];
    dfs(&g, 0, 0, 0, &mut [0; 26], &mut depth, &mut parent, &mut info);
    for j in 0..mx - 1 {
        for i in 0..n {
            let pp = parent[j][i];
            if parent[j][i] != n {
                parent[j + 1][i] = parent[j][pp];
            }
        }
    }
    queries.into_iter().map(|query| {
        let (mut u, mut v) = (query[0] as usize, query[1] as usize);
        if depth[u] > depth[v] { std::mem::swap(&mut u, &mut v); }
        let diff = depth[v] - depth[u];
        for i in 0..mx {
            if diff >> i & 1 == 1 {
                v = parent[i][v];
            }
        }
        if u != v {
            for i in (0..mx).rev() {
                if parent[i][u] != parent[i][v] {
                    u = parent[i][u];
                    v = parent[i][v];
                }
            }
            u = parent[0][u];
        }
        let lca = u;
        let (u, v) = (query[0] as usize, query[1] as usize);
        let (sum, max) = info[u].iter().zip(&info[v]).zip(&info[lca]).map(|((a, b), c)| *a + *b - *c * 2).fold((0, 0), |(sum, max), num| (sum + num, max.max(num)));
        sum - max
    }).collect()
}


/// tarjan 求LCA，离线算法
/// 或者 tarjan就求 LCA，然后用 root_cnt[u] + root_cnt[v] - 2 * root_cnt[lca] 也可以
pub fn min_operations_queries3(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    struct Tarjan<'a> {
        g: &'a Vec<Vec<(usize, i32)>>,
        query: &'a Vec<Vec<(usize, usize)>>,
        vis: Vec<bool>,
        fa: Vec<usize>,
        fa_cnt: Vec<[i32; 26]>,
        root_cnt: Vec<[i32; 26]>,
        // 从根节点到当前节点的Info
        result: Vec<i32>,
    }

    impl<'a> Tarjan<'a> {
        fn get(&mut self, x: usize) -> usize {
            if self.fa[x] != x {
                let p = self.fa[x];
                self.fa[x] = self.get(p);
                for i in 0..26 { self.fa_cnt[x][i] += self.fa_cnt[p][i] }
            }
            self.fa[x]
        }
        fn tarjan(&mut self, u: usize) {
            self.vis[u] = true;
            for &(v, w) in &self.g[u] {
                if !self.vis[v] {
                    self.root_cnt[v] = self.root_cnt[u].clone();
                    self.root_cnt[v][w as usize] += 1;
                    self.tarjan(v);
                    self.fa[v] = u;
                    self.fa_cnt[v][w as usize] = 1;
                }
            }

            for &(v, i) in &self.query[u] {
                if self.vis[v] { // 另一个点已经访问过了，可以得出答案
                    let lca = self.get(v);
                    let (sum, max) = self.root_cnt[u].iter().zip(&self.fa_cnt[v]).zip(&self.root_cnt[lca]).map(|((a, b), c)| *a + *b - *c).fold((0, 0), |(sum, max), num| (sum + num, max.max(num)));
                    self.result[i] = sum - max;
                }
            }
        }
    }
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2] - 1);
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut q = vec![vec![]; n];
    let qlen = queries.len();
    for (i, query) in queries.into_iter().enumerate() {
        let (u, v) = (query[0] as usize, query[1] as usize);
        q[u].push((v, i));
        q[v].push((u, i));
    }
    let mut t = Tarjan { g: &g, query: &q, vis: vec![false; n], fa: (0..n).collect(), fa_cnt: vec![[0; 26]; n], root_cnt: vec![[0; 26]; n], result: vec![0; qlen] };
    t.tarjan(0);
    t.result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(8, vec![vec![1, 2, 6], vec![1, 3, 4], vec![2, 4, 6], vec![2, 5, 3], vec![3, 6, 6], vec![3, 0, 8], vec![7, 0, 2]], vec![vec![4, 6], vec![0, 4], vec![6, 5], vec![7, 4]]), vec![1, 2, 2, 3]);
        assert_eq!(func(7, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 2], vec![4, 5, 2], vec![5, 6, 2]], vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]]), vec![0, 0, 1, 3]);
    }
    test(min_operations_queries);
    test(min_operations_queries2);
    test(min_operations_queries3);
}
