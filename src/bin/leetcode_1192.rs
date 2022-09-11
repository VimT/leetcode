//! 查找集群内的「关键连接」


struct Tarjan {
    edge: Vec<Vec<usize>>,
    // 追溯值：以当前节点作为搜索树的根节点，能访问到的所有节点中，时间戳最小的值
    low: Vec<usize>,
    // 深度优先搜索遍历时结点 u 被搜索的次序（时间戳）
    dfn: Vec<usize>,
    bridge: Vec<(usize, usize)>,
}

impl Tarjan {
    fn new(edge: Vec<Vec<usize>>) -> Self {
        let len = edge.len();
        Self {
            edge,
            low: vec![0; len],
            dfn: vec![0; len],
            bridge: vec![],
        }
    }
    fn tarjan(&mut self, now: usize, cnt: usize) {
        self.dfn[now] = cnt;
        self.low[now] = cnt;
        for i in self.edge[now].clone().into_iter().rev() {
            if self.dfn[i] == 0 {
                self.tarjan(i, cnt + 1);
                self.low[now] = self.low[now].min(self.low[i]);

                // 是桥（找割点是 >=）
                if self.low[i] > self.dfn[now] {
                    self.bridge.push((now, i));
                }
            } else if self.dfn[i] != self.dfn[now] - 1 {
                self.low[now] = self.low[now].min(self.dfn[i]);
            }
        }
    }
}


/// 双连通分量，割点，桥
/// leetcode_lcp_54 找割点，本题找桥
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = n as usize;
    let mut edge = vec![vec![]; len];
    for conn in &connections {
        edge[conn[0] as usize].push(conn[1] as usize);
        edge[conn[1] as usize].push(conn[0] as usize);
    }
    let mut tj = Tarjan::new(edge);
    for i in 0..len {
        if tj.dfn[i] == 0 {
            tj.tarjan(i, 1);
        }
    }
    tj.bridge.into_iter().map(|x| vec![x.0 as i32, x.1 as i32]).collect()
}

fn main() {
    fn test(func: fn(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]), vec![vec![1, 3]]);
        assert_eq!(func(2, vec![vec![0, 1]]), vec![vec![0, 1]]);
    }
    test(critical_connections);
}
