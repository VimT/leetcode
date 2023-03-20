//! 收集树上所有苹果的最少时间

use leetcode::graph::Graph;

pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    let n = n as usize;
    let mut graph = Graph::new(n, n * 2);
    for edge in edges {
        graph.add(edge[0] as usize, edge[1] as usize, ());
        graph.add(edge[1] as usize, edge[0] as usize, ());
    }
    // 采集u为根的所有水果并返回u
    fn dfs(g: &Graph, u: usize, fa: usize, has_apple: &Vec<bool>) -> i32 {
        let mut result = 0;
        let mut apple = has_apple[u];
        for (v, _) in g.neigh(u) {
            if v != fa {
                let vv = dfs(g, v, u, has_apple);
                if vv >= 0 {
                    apple = true;
                    result += vv + 2;
                }
            }
        }
        if apple { result } else { -1 }
    }
    dfs(&graph, 0, usize::MAX, &has_apple).max(0)
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32) {
        assert_eq!(func(7, vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]], vec![false, false, true, false, true, true, false]), 8);
        assert_eq!(func(7, vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]], vec![false, false, true, false, false, true, false]), 6);
        assert_eq!(func(7, vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]], vec![false, false, false, false, false, false, false]), 0);
    }
    test(min_time);
}
