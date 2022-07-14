//! 夺回据点


struct Tarjan {
    edge: Vec<Vec<usize>>,
    // 遍历过的数
    st: Vec<usize>,
    // 追溯值：以当前节点作为搜索树的根节点，能访问到的所有节点中，时间戳最小的值
    low: Vec<usize>,
    // 深度优先搜索遍历时结点 u 被搜索的次序（时间戳）
    dfn: Vec<usize>,
    is_cut: Vec<bool>,
    // 由一个割点，分成的几个part
    bcc: Vec<Vec<usize>>,
}

impl Tarjan {
    fn new(edge: Vec<Vec<usize>>) -> Self {
        let len = edge.len();
        Self {
            edge,
            st: vec![],
            low: vec![0; len],
            dfn: vec![0; len],
            is_cut: vec![false; len],
            bcc: vec![],
        }
    }
    fn tarjan(&mut self, now: usize, cnt: usize) {
        let mut part = (cnt > 1) as i32;
        self.st.push(now);
        self.dfn[now] = cnt;
        self.low[now] = cnt;
        for i in self.edge[now].clone().into_iter().rev() {
            if self.dfn[i] == 0 {
                self.tarjan(i, cnt + 1);
                self.low[now] = self.low[now].min(self.low[i]);

                // 是桥
                if self.low[i] >= self.dfn[now] {
                    part += 1;
                    if part == 2 {
                        self.is_cut[now] = true;
                    }
                    let mut a = vec![now];
                    loop {
                        let last = self.st.pop().unwrap();
                        a.push(last);
                        if last == i { break; }
                    }
                    self.bcc.push(a);
                }
            } else if self.dfn[i] != self.dfn[now] - 1 {
                self.low[now] = self.low[now].min(self.dfn[i]);
            }
        }
    }
}


pub fn minimum_cost(cost: Vec<i32>, roads: Vec<Vec<i32>>) -> i64 {
    let len = cost.len();
    let mut edge = vec![vec![]; len];
    for road in &roads {
        edge[road[0] as usize].push(road[1] as usize);
        edge[road[1] as usize].push(road[0] as usize);
    }
    let mut tj = Tarjan::new(edge);
    for i in 0..len {
        if tj.dfn[i] == 0 {
            tj.tarjan(i, 1);
        }
    }
    let mut result = 0;
    let mut leaf_cnt = 0;
    let mut max_leaf_cost = 0;
    for b in &tj.bcc {
        let mut cut_cnt = 0;
        let mut min_cost = 1e9 as i64;
        for &t in b {
            if tj.is_cut[t] {
                cut_cnt += 1;
            } else {
                min_cost = min_cost.min(cost[t] as i64);
            }
        }
        if cut_cnt <= 1 {
            leaf_cnt += 1;
            result += min_cost;
            max_leaf_cost = max_leaf_cost.max(min_cost);
        }
    }
    if leaf_cnt > 1 {
        result -= max_leaf_cost;
    }
    result
}

fn main() {
    assert_eq!(minimum_cost(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![1, 2], vec![2, 4], vec![2, 5]]), 6);
    assert_eq!(minimum_cost(vec![3, 2, 1, 4], vec![vec![0, 2], vec![2, 3], vec![3, 1]]), 2);
}
