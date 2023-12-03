//! 将石头分散到网格图的最少移动次数

use std::collections::VecDeque;

/// 全排列
pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut can = vec![];
    let mut empty = vec![];
    for i in 0..3 {
        for j in 0..3 {
            for _ in 1..grid[i][j] { can.push((i * 3 + j) as i32); }
            if grid[i][j] == 0 { empty.push((i * 3 + j) as i32); }
        }
    }
    fn backtrack(can_use: &Vec<i32>, empty: &mut Vec<i32>, first: usize, result: &mut i32) {
        if first == empty.len() {
            *result = (*result).min(can_use.iter().zip(empty.iter()).map(|(&a, &b)| {
                let (x0, y0) = (a % 3, a / 3);
                let (x, y) = (b % 3, b / 3);
                (x - x0).abs() + (y - y0).abs()
            }).sum::<i32>());
        }
        for i in first..empty.len() {
            empty.swap(i, first);
            backtrack(can_use, empty, first + 1, result);
            empty.swap(i, first);
        }
    }
    let mut result = i32::MAX;
    backtrack(&can, &mut empty, 0, &mut result);
    result
}

/// dp[i ^ (1<<k)][j] 表示前i个可用石头，第i个填补了j状态的第k个空位后的 最小代价
pub fn minimum_moves2(grid: Vec<Vec<i32>>) -> i32 {
    let mut can = vec![];
    let mut start = 0;
    for i in 0..3 {
        for j in 0..3 {
            for _ in 1..grid[i][j] { can.push((i, j)); }
            if grid[i][j] == 0 { start |= 1 << (i * 3 + j); }
        }
    }
    let mut dp = vec![i32::MAX / 2; 1 << 10];
    dp[start] = 0;
    for (x, y) in can {
        for j in 0..1 << 10 {
            for k in 0..10 {
                if j >> k & 1 == 1 {
                    let (x1, y1) = (k / 3, k % 3);
                    dp[j ^ (1 << k)] = dp[j ^ (1 << k)].min(dp[j] + (x as i32 - x1).abs() + (y as i32 - y1).abs());
                }
            }
        }
    }
    dp[0]
}

/// 最小费用最大流
/// 从每个大于 1 的格子向每个等于 0 的格子连边，容量为 1，费用为两个格子之间的曼哈顿距离。
/// 从超级源点向每个大于 1 的格子连边，容量为格子的值减一（即移走的石子数），费用为 0。
/// 从每个等于 0 的格子向超级汇点连边，容量 1（即移入的石子数），费用为 0。
pub fn minimum_moves3(grid: Vec<Vec<i32>>) -> i32 {
    #[derive(Debug, Clone)]
    struct Edge {
        to: usize,
        // 反边在邻接表的下标
        rid: usize,
        cap: i32,
        cost: i32,
    }
    struct Graph {
        g: Vec<Vec<Edge>>,
        fa: Vec<(usize, usize)>,
        dist: Vec<i32>,
    }
    impl Graph {
        fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
            let rid = self.g[to].len();
            self.g[from].push(Edge { to, rid, cap, cost });
            let rid = self.g[from].len() - 1;
            self.g[to].push(Edge { to: from, rid, cap: 0, cost: -cost });  // 在无向图上0换成cap
        }
        fn spfa(&mut self, start: usize, end: usize) -> bool {
            self.dist.fill(i32::MAX);
            self.dist[start] = 0;
            let mut inq = vec![false; self.g.len()];
            inq[start] = true;
            let mut q = VecDeque::new();
            q.push_back(start);
            while !q.is_empty() {
                let v = q.pop_front().unwrap();
                inq[v] = false;
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap == 0 { continue; }
                    let w = e.to;
                    let new_d = self.dist[v] + e.cost;
                    if new_d < self.dist[w] {
                        self.dist[w] = new_d;
                        self.fa[w] = (v, i);
                        if !inq[w] {
                            q.push_back(w);
                            inq[w] = true;
                        }
                    }
                }
            }
            self.dist[end] < i32::MAX
        }
        fn ek(&mut self, start: usize, end: usize) -> (i32, i32) {
            let mut max_flow = 0;
            let mut min_cost = 0;
            while self.spfa(start, end) {
                // 沿 start-end 的最短路尽量增广
                let mut min_f = i32::MAX;
                let mut v = end;
                while v != start {
                    let (fa, i) = self.fa[v];
                    min_f = min_f.min(self.g[fa][i].cap);
                    v = fa;
                }
                v = end;
                while v != start {
                    let (fa, i) = self.fa[v];
                    self.g[fa][i].cap -= min_f;
                    let rid = self.g[fa][i].rid;
                    self.g[v][rid].cap += min_f;
                    v = fa;
                }
                max_flow += min_f;
                min_cost += self.dist[end] * min_f;
            }
            (max_flow, min_cost)
        }
    }

    let mut g = Graph { g: vec![vec![]; 11], fa: vec![(0, 0); 11], dist: vec![0; 11] };
    let start = 9;
    let end = 10;
    for i in 0..3 {
        for j in 0..3 {
            let v = grid[i][j];
            if v > 1 {
                g.add_edge(start, i * 3 + j, v - 1, 0);
                for k in 0..3 {
                    for l in 0..3 {
                        if grid[k][l] == 0 {
                            g.add_edge(i * 3 + j, k * 3 + l, 1, (k as i32 - i as i32).abs() + (l as i32 - j as i32).abs());
                        }
                    }
                }
            } else if v == 0 {
                g.add_edge(i * 3 + j, end, 1, 0);
            }
        }
    }
    g.ek(start, end).1
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![3, 2, 0], vec![0, 1, 0], vec![0, 3, 0]]), 7);
        assert_eq!(func(vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]]), 4);
        assert_eq!(func(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]]), 3);
    }
    test(minimum_moves);
    test(minimum_moves2);
    test(minimum_moves3);
}
