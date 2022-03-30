//! 连通两组点的最小成本

use std::collections::VecDeque;

// km
struct Hungarian<T> {
    n: usize,
    // 左集合对应的匹配点
    matchx: Vec<i32>,
    // 右集合对应的匹配点
    matchy: Vec<i32>,
    // 连接右集合的左点
    pre: Vec<usize>,
    // 拜访数组 左
    visx: Vec<bool>,
    // 拜访数组 右
    visy: Vec<bool>,
    lx: Vec<T>,
    ly: Vec<T>,
    g: Vec<Vec<T>>,
    slack: Vec<T>,
    q: VecDeque<usize>,
}

impl Hungarian<i32> {
    fn new(n: usize, m: usize) -> Self {
        let maxn = n.max(m);
        Hungarian {
            n: maxn,
            matchx: vec![-1; maxn],
            matchy: vec![-1; maxn],
            pre: vec![0; maxn],
            visx: vec![false; maxn],
            visy: vec![false; maxn],
            lx: vec![i32::MIN; maxn],
            ly: vec![0; maxn],
            g: vec![vec![0; maxn]; maxn],
            slack: vec![0; maxn],
            q: Default::default(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: i32) {
        // 负值还不如不匹配 因此设为0不影响
        self.g[u][v] = w.max(0);
    }

    fn check(&mut self, mut v: usize) -> bool {
        self.visy[v] = true;
        if self.matchy[v] != -1 {
            self.q.push_back(self.matchy[v] as usize);
            self.visx[self.matchy[v] as usize] = true;
            return false;
        }
        loop {
            // 找到新的未匹配点 更新匹配点 pre 数组记录着"非匹配边"上与之相连的点
            self.matchy[v] = self.pre[v] as i32;
            let tmp = self.matchx[self.pre[v]];
            self.matchx[self.pre[v]] = v as i32;
            if tmp == -1 { break; }
            v = tmp as usize;
        }
        true
    }

    fn bfs(&mut self, i: usize) {
        self.q.clear();
        self.q.push_back(i);

        loop {
            while !self.q.is_empty() {
                let u = self.q.pop_front().unwrap();
                for v in 0..self.n {
                    if !self.visy[v] {
                        let delta = self.lx[u] + self.ly[v] - self.g[u][v];
                        if self.slack[v] >= delta {
                            self.pre[v] = u;
                            if delta != 0 {
                                self.slack[v] = delta;
                            } else if self.check(v) {
                                // delta=0 代表有机会加入相等子图 找增广路
                                // 找到就return 重建交错树
                                return;
                            }
                        }
                    }
                }
            }
            // 没有增广路 修改顶标
            let mut a = i32::MAX;
            for j in 0..self.n {
                if !self.visy[j] {
                    a = a.min(self.slack[j]);
                }
            }
            for j in 0..self.n {
                if self.visx[j] {//S
                    self.lx[j] -= a;
                }
                if self.visy[j] { //T
                    self.ly[j] += a;
                } else { //T'
                    self.slack[j] -= a;
                }
            }
            for j in 0..self.n {
                if !self.visy[j] && self.slack[j] == 0 && self.check(j) {
                    return;
                }
            }
        }
    }

    fn solve(&mut self) -> i32 {
        // 初始顶标
        for i in 0..self.n {
            for j in 0..self.n {
                self.lx[i] = self.lx[i].max(self.g[i][j]);
            }
        }
        for i in 0..self.n {
            self.slack.fill(i32::MAX);
            self.visx.fill(false);
            self.visy.fill(false);
            self.bfs(i);
        }

        let mut res = 0;
        // custom
        for i in 0..self.n {
            if self.g[i][self.matchx[i] as usize] > 0 {
                res += self.g[i][self.matchx[i] as usize];
            } else {
                self.matchx[i] = -1;
            }
        }
        res
    }
}

/// 二分图最大权匹配
pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
    let n = cost.len();
    let m = cost[0].len();
    let mut hg = Hungarian::new(n, m);
    let mut lmin = vec![i32::MAX; n];
    let mut rmin = vec![i32::MAX; m];
    for i in 0..n {
        for j in 0..m {
            lmin[i] = lmin[i].min(cost[i][j]);
            rmin[j] = rmin[j].min(cost[i][j]);
        }
    }
    let ans: i32 = lmin.iter().sum::<i32>() + rmin.iter().sum::<i32>();
    for i in 0..n {
        for j in 0..m {
            hg.add_edge(i, j, lmin[i] + rmin[j] - cost[i][j]);
        }
    }
    ans - hg.solve()
}


pub fn connect_two_groups_dp(cost: Vec<Vec<i32>>) -> i32 {
    let m = cost.len();
    let n = cost[0].len();
    let mut cost_matrix = vec![vec![0; 1 << n]; m];
    for k in 0..m {
        for i in 0..1 << n {
            let mut sum = 0;
            for j in 0..n {
                if i & (1 << j) > 0 {
                    sum += cost[k][j];
                }
            }
            cost_matrix[k][i] = sum;
        }
    }
    let mut dp = vec![vec![i32::MAX; 1 << n]; m];
    dp[0] = cost_matrix[0].clone();
    for i in 1..m {
        for k in 1..1 << n {
            // 首先将第 i 行只选取一个元素的情况都考虑一遍
            // 这样做的目的是保证第 i 行至少选取了一个元素
            for j in 0..n {
                dp[i][k | (1 << j)] = dp[i][k | (1 << j)].min(dp[i - 1][k] + cost[i][j]);
            }
            // rest 表示截至第 i 行还没被选过的列
            let rest = (1 << n) - 1 - k;
            // 只遍历没选过的列的所有组合
            let mut j = rest;
            while j >= 1 {
                dp[i][j | k] = dp[i][j | k].min(dp[i - 1][k] + cost_matrix[i][j]);
                j = rest & (j - 1);
            }
        }
    }
    dp[m - 1][(1 << n) - 1]
}


fn main() {
    assert_eq!(connect_two_groups_dp(vec![vec![7, 38], vec![43, 44], vec![72, 2], vec![64, 48], vec![90, 32], vec![10, 34], vec![50, 62], vec![99, 20], vec![39, 24]]), 236);
    assert_eq!(connect_two_groups_dp(vec![vec![80, 96, 44], vec![38, 11, 8], vec![37, 73, 77], vec![77, 33, 57], vec![8, 72, 65], vec![48, 17, 66], vec![58, 62, 80], vec![70, 68, 39]]), 244);
    assert_eq!(connect_two_groups_dp(vec![vec![15, 96], vec![36, 2]]), 17);
    assert_eq!(connect_two_groups_dp(vec![vec![1, 3, 5], vec![4, 1, 1], vec![1, 5, 3]]), 4);
    assert_eq!(connect_two_groups_dp(vec![vec![2, 5, 1], vec![3, 4, 7], vec![8, 1, 2], vec![6, 2, 4], vec![3, 8, 8]]), 10);
}

