//! 设计可以求最短路径的图类

use std::collections::BinaryHeap;

struct Graph {
    g: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut g = vec![vec![]; n as usize];
        for edge in edges {
            g[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }
        Self { g }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.g[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut q = BinaryHeap::new();
        let mut dist = vec![i32::MAX; self.g.len()];
        dist[node1 as usize] = 0;
        q.push((0, node1 as usize));
        while !q.is_empty() {
            let (_, u) = q.pop().unwrap();
            if u as i32 == node2 {
                return dist[u];
            }
            for &(v, cost) in &self.g[u] {
                if dist[v] > dist[u] + cost {
                    dist[v] = dist[u] + cost;
                    q.push((-dist[v], v));
                }
            }
        }
        -1
    }
}

struct Graph2 {
    g: Vec<Vec<i32>>,
}

impl Graph2 {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        // floyd: dp[k][i][j] 表示从i到j的中间路径上（包括i和j）的节点最大为k时的 最短路
        // 两种情况：1. 从i到j的最短路最多为k-1。 2. 从i到j的最短路最多为k（k一定是中间节点）
        // dp[k][i][j] = min( dp[k-1][i][j], dp[k-1][i][k] + dp[k-1][k][j] )
        let mut g = vec![vec![i32::MAX / 4; n]; n];
        for i in 0..n { g[i][i] = 0; }
        for e in edges { g[e[0] as usize][e[1] as usize] = e[2]; }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }
        Self { g }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let n = self.g.len();
        let (x, y, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        if w >= self.g[x][y] { return; }
        for i in 0..n {
            for j in 0..n {
                self.g[i][j] = self.g[i][j].min(self.g[i][x] + w + self.g[y][j]);
            }
        }
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let result = self.g[node1 as usize][node2 as usize];
        if result >= i32::MAX / 4 { -1 } else { result }
    }
}


fn main() {
    macro_rules! test {
        ($c:tt) => {
            let mut g = $c::new(4, vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]]);
            assert_eq!(g.shortest_path(3, 2), 6); // 返回 6 。从 3 到 2 的最短路径如第一幅图所示：3 -> 0 -> 1 -> 2 ，总代价为 3 + 2 + 1 = 6 。
            assert_eq!(g.shortest_path(0, 3), -1); // 返回 -1 。没有从 0 到 3 的路径。
            g.add_edge(vec![1, 3, 4]); // 添加一条节点 1 到节点 3 的边，得到第二幅图。
            assert_eq!(g.shortest_path(0, 3), 6); // 返回 6 。从 0 到 3 的最短路径为 0 -> 1 -> 3 ，总代价为 2 + 4 = 6 。
        };
    }
    test!(Graph);
    test!(Graph2);
}
