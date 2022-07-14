//! 节点序列的最大得分

/// 设序列为 a-x-y-b，枚举 edges 中的每条边，作为序列中的 x-y。
pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let len = scores.len();
    let mut g = vec![vec![]; len];
    for edge in &edges {
        g[edge[0] as usize].push((-scores[edge[1] as usize], edge[1] as usize));
        g[edge[1] as usize].push((-scores[edge[0] as usize], edge[0] as usize));
    }
    for v in &mut g {
        v.sort_unstable();
        if v.len() > 3 {
            unsafe { v.set_len(3); }
        }
    }
    let mut result = -1;
    for edge in edges {
        let x = edge[0] as usize;
        let y = edge[1] as usize;
        for (_, a) in &g[x] {
            for (_, b) in &g[y] {
                if *a != y && *b != x && *a != *b {
                    result = result.max(scores[x] + scores[y] + scores[*a] + scores[*b]);
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(maximum_score(vec![5, 2, 9, 8, 4], vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 2], vec![1, 3], vec![2, 4]]), 24);
    assert_eq!(maximum_score(vec![9, 20, 6, 4, 11, 12], vec![vec![0, 3], vec![5, 3], vec![2, 4], vec![1, 3]]), -1);
}
