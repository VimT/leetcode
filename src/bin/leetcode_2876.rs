//! 有向图访问计数

use std::collections::VecDeque;

/// 内向基环树
pub fn count_visited_nodes(g: Vec<i32>) -> Vec<i32> {
    let len = g.len();
    let mut rg = vec![vec![]; len]; // 反图
    let mut indegree = vec![0; len];
    for (i, &j) in g.iter().enumerate() {
        rg[j as usize].push(i);
        indegree[j as usize] += 1;
    }

    // 拓扑排序找所有环
    let mut q = VecDeque::new();
    for i in 0..len {
        if indegree[i] == 0 {
            q.push_back(i);
        }
    }
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        let v = g[u] as usize;
        indegree[v] -= 1;
        if indegree[v] == 0 {
            q.push_back(v);
        }
    }

    let mut result = vec![0; len];
    fn rdfs(rg: &Vec<Vec<usize>>, indegree: &Vec<i32>, u: usize, depth: i32, result: &mut Vec<i32>) {
        result[u] = depth;
        for &v in &rg[u] {
            if indegree[v] == 0 { // 如果是树枝
                rdfs(rg, indegree, v, depth + 1, result);
            }
        }
    }
    for i in 0..len {
        if indegree[i] > 0 {
            let mut ring = vec![i];
            indegree[i] = -1;
            let mut u = g[i] as usize;
            while u != i {
                indegree[u] = -1;  // 基环上的点入度标记为0，避免重复访问
                ring.push(u);
                u = g[u] as usize;
            }
            for &u in &ring {
                rdfs(&rg, &indegree, u, ring.len() as i32, &mut result);
            }
        }
    }

    result
}


/// Tarjan (也适用 n 个节点，m 条边的情况）
pub fn count_visited_nodes2(g: Vec<i32>) -> Vec<i32> {
    let len = g.len();
    let mut cnt = 1;
    let mut dfn = vec![0; len];
    let mut result = vec![0; len];

    fn dfs(cnt: &mut i32, u: usize, g: &Vec<i32>, dfn: &mut Vec<i32>, result: &mut Vec<i32>) -> i32 {
        if result[u] != 0 { return result[u]; }
        if dfn[u] != 0 { // 说明是个环
            let mut x = u;
            let c = *cnt - dfn[x];
            while result[x] == 0 {
                result[x] = c;
                x = g[x] as usize;
            }
            return result[x];
        }
        dfn[u] = *cnt;
        *cnt += 1;
        let v = g[u] as usize;
        dfs(cnt, v, g, dfn, result);
        if result[u] == 0 { result[u] = result[v] + 1; }
        result[u]
    }
    for i in 0..len {
        dfs(&mut cnt, i, &g, &mut dfn, &mut result);
    }
    result
}

fn main() {
    fn test(func: fn(edges: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 2, 0, 0]), vec![3, 3, 3, 4]);
        assert_eq!(func(vec![1, 2, 3, 4, 0]), vec![5, 5, 5, 5, 5]);
    }
    test(count_visited_nodes);
}
