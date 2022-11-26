//! 树上最大得分和路径


use std::collections::VecDeque;

/// BFS组树 + 回溯
pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, mut amount: Vec<i32>) -> i32 {
    let len = amount.len();
    let mut g = vec![vec![]; len];
    let mut parents = vec![0; len];
    let mut subs = vec![vec![]; len];

    for edge in &edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }

    let mut q = VecDeque::new();
    q.push_back(0);
    let mut seen = vec![false; len];
    seen[0] = true;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for &v in &g[u] {
            if !seen[v] {
                parents[v] = u;
                subs[u].push(v);
                seen[v] = true;
                q.push_back(v);
            }
        }
    }

    fn dfs(parent: &Vec<usize>, sub: &Vec<Vec<usize>>, amount: &mut Vec<i32>, a: usize, b: usize, score: i32, result: &mut i32) {
        if sub[a].len() == 0 {
            *result = (*result).max(score);
            return;
        }
        let b_next = parent[b];
        for &a_next in &sub[a] {
            let tmp = amount[b_next];
            let add = if a_next == b_next {
                amount[a_next] / 2
            } else { amount[a_next] };

            amount[b_next] = 0;
            dfs(parent, sub, amount, a_next, b_next, score + add, result);
            amount[b_next] = tmp;
        }
    }
    let mut result = i32::MIN;
    let score = amount[0];
    amount[bob as usize] = 0;
    dfs(&parents, &subs, &mut amount, 0, bob as usize, score, &mut result);
    result
}

fn main() {
    assert_eq!(most_profitable_path(vec![vec![0,2],vec![0,5],vec![1,3],vec![1,5],vec![2,4]], 4, vec![5018,8388,6224,3466,3808,3456]), 20328);
    assert_eq!(most_profitable_path(vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]], 3, vec![-2, 4, 2, -4, 6]), 6);
    assert_eq!(most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 2350]), -7280);
}
