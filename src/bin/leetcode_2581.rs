//! 统计可能的树根数目

use std::collections::HashSet;
use leetcode::graph::Graph;

/// 换根dp，两次dfs，第一次计算信息info，第二次考虑根从u向v转移时的变化
pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = edges.len() + 1;
    let mut g = Graph::new(n, 2 * n);
    let guess: HashSet<(i32, i32)> = guesses.into_iter().map(|x| (x[0], x[1])).collect();
    for edge in edges {
        let a = guess.contains(&(edge[0], edge[1])) as i32;
        let b = guess.contains(&(edge[1], edge[0])) as i32;
        g.add(edge[0] as usize, edge[1] as usize, (a, b));
        g.add(edge[1] as usize, edge[0] as usize, (b, a));
    }

    fn dfs(g: &Graph<(i32, i32)>, u: usize, fa: usize) -> i32 {
        let mut m = 0;
        for (v, &(x, _)) in g.neigh(u) {
            if v != fa {
                let a = dfs(g, v, u);
                m += x + a;
            }
        }
        m
    }

    fn dfs2(g: &Graph<(i32, i32)>, u: usize, fa: usize, m: i32, k: i32, result: &mut i32) {
        if m >= k {
            *result += 1;
        }
        for (v, &(a, b)) in g.neigh(u) {
            if v != fa {
                let va = m - a + b;
                dfs2(g, v, u, va, k, result);
            }
        }
    }
    let mut result = 0;
    let m = dfs(&g, 0, n);
    dfs2(&g, 0, n, m, k, &mut result);
    result
}

fn main() {
    fn test(func: fn(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![5, 4], vec![6, 0], vec![4, 7], vec![8, 5], vec![2, 9], vec![10, 0]], vec![vec![8, 5], vec![9, 2]], 1), 2);
        assert_eq!(func(vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]], vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]], 3), 3);
        assert_eq!(func(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![1, 0], vec![3, 4], vec![2, 1], vec![3, 2]], 1), 5);
    }
    test(root_count);
}
