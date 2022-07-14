//! 校园自行车分配 II

use std::collections::VecDeque;

fn dis(a: &[i32], b: &[i32]) -> i32 {
    return (a[0] - b[0]).abs() + (a[1] - b[1]).abs();
}

pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
    let len = bikes.len();
    let mut dp = vec![i32::MAX; 1 << len];
    dp[0] = 0;
    let mut result = i32::MAX;
    for (bi, worker) in workers.iter().enumerate() {
        for i in 0..1usize << len {
            if i.count_ones() == bi as u32 {
                for j in 0..len {
                    if i >> j & 1 == 0 {
                        dp[i | (1 << j)] = dp[i | (1 << j)].min(dp[i] + dis(&bikes[j], worker));
                        if bi == workers.len() - 1 {
                            result = result.min(dp[i | (1 << j)]);
                        }
                    }
                }
            }
        }
    }
    result
}

struct KM {
    n: usize,
    // 左集合对应的匹配点
    match_x: Vec<usize>,
    // 右集合对应的匹配点
    match_y: Vec<usize>,
    // 连接右集合的左点
    pre: Vec<usize>,
    // 左拜访数组
    vis_x: Vec<bool>,
    // 右拜访数组
    vis_y: Vec<bool>,
    // 可行顶标，每个节点分配权值 l(i)，对所有边满足w(u,v) <= l(u) + l(v)
    lx: Vec<i32>,
    ly: Vec<i32>,
    graph: Vec<Vec<i32>>,
    slack: Vec<i32>,
    queue: VecDeque<usize>,
}

impl KM {
    fn new(n: usize, graph: Vec<Vec<i32>>) -> Self {
        return Self {
            n,
            match_x: vec![usize::MAX; n],
            match_y: vec![usize::MAX; n],
            pre: vec![0; n],
            vis_x: vec![false; n],
            vis_y: vec![false; n],
            lx: vec![i32::MIN; n],
            ly: vec![0; n],
            graph,
            slack: vec![0; n],
            queue: Default::default(),
        };
    }

    fn check(&mut self, mut v: usize) -> bool {
        self.vis_y[v] = true;
        if self.match_y[v] != usize::MAX {
            self.queue.push_back(self.match_y[v]);
            self.vis_x[self.match_y[v]] = true;
            return false;
        }
        // 找到新的未匹配点，更新匹配点pre数组，记录着“非匹配边”上与之相连的点
        while v != usize::MAX {
            self.match_y[v] = self.pre[v];
            let tmp = self.match_x[self.pre[v]];
            self.match_x[self.pre[v]] = v;
            v = tmp;
        }
        true
    }

    fn bfs(&mut self, i: usize) {
        self.queue.clear();
        self.queue.push_back(i);
        self.vis_x[i] = true;
        loop {
            while !self.queue.is_empty() {
                let u = self.queue.pop_front().unwrap();
                for v in 0..self.n {
                    if !self.vis_y[v] {
                        let delta = self.lx[u] + self.ly[v] - self.graph[u][v];
                        if self.slack[v] >= delta {
                            self.pre[v] = u;
                            if delta > 0 {
                                self.slack[v] = delta;
                            } else if self.check(v) {
                                // delta = 0 表示有机会加入相等子图，找增广路
                                // 找到就return 重建交错树
                                return
                            }
                        }
                    }
                }
            }
            // 没有增广路，修改顶标
            let mut a = i32::MAX;
            for j in 0..self.n {
                if !self.vis_y[j] {
                    a = a.min(self.slack[j]);
                }
            }
            for j in 0..self.n {
                // S
                if self.vis_x[j] {
                    self.lx[j] -= a;
                }
                // T
                if self.vis_y[j] {
                    self.ly[j] += a;
                }
                // T'
                else {
                    self.slack[j] -= a;
                }
            }
            for j in 0..self.n {
                if !self.vis_y[j] && self.slack[j] == 0 && self.check(j) {
                    return;
                }
            }
        }
    }

    fn get_maximum_weight(&mut self) -> i32 {
        // 初始顶标
        for i in 0..self.n {
            for j in 0..self.n {
                self.lx[i] = self.lx[i].max(self.graph[i][j]);
            }
        }
        for i in 0..self.n {
            self.slack.fill(i32::MAX);
            self.vis_x.fill(false);
            self.vis_y.fill(false);
            self.bfs(i);
        }
        let mut result = 0;
        for i in 0..self.n {
            result += self.graph[i][self.match_x[i]];
        }
        result
    }
}

/// 匈牙利算法又称为 KM 算法，可以在 O(n^3) 时间内求出二分图的 最大权完美匹配。
pub fn assign_bikes2(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
    let n = workers.len();
    let m = bikes.len();
    let mut graph = vec![vec![0; m]; m];
    for i in 0..n {
        for j in 0..m {
            // 求最小匹配，取反
            graph[i][j] = -dis(&workers[i], &bikes[j]);
        }
    }
    let mut km = KM::new(m, graph);
    -km.get_maximum_weight()
}

fn main() {
    fn test(func: fn(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![239, 904], vec![191, 103], vec![260, 117], vec![86, 78], vec![747, 62]],
                        vec![vec![660, 8], vec![431, 772], vec![78, 576], vec![894, 481], vec![451, 730], vec![155, 28]]), 1886);
        assert_eq!(func(vec![vec![0, 0], vec![2, 1]], vec![vec![1, 2], vec![3, 3]]), 6);
        assert_eq!(func(vec![vec![0, 0], vec![1, 1], vec![2, 0]], vec![vec![1, 0], vec![2, 2], vec![2, 1]]), 4);
        assert_eq!(func(vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0]], vec![vec![0, 999], vec![1, 999], vec![2, 999], vec![3, 999], vec![4, 999]]), 4995);
    }
    test(assign_bikes);
    test(assign_bikes2);
}
