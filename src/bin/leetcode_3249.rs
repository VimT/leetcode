//! 统计好节点的数目

pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        g[u].push(v);
        g[v].push(u);
    }
    fn dfs(g: &Vec<Vec<usize>>, u: usize, fa: usize, result: &mut i32) -> i32 {
        let mut size = 0;
        let mut prev = 0;
        let mut ok = true;
        for &v in &g[u] {
            if v == fa { continue; }
            let sub = dfs(g, v, u, result);
            if prev != 0 && sub != prev { ok = false; }
            size += sub;
            prev = sub;
        }
        if ok { *result += 1; }
        size + 1
    }
    let mut result = 0;
    dfs(&g, 0, n, &mut result);
    result
}

fn main() {
    fn test(func: fn(edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]]), 7);
        assert_eq!(func(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![0, 5], vec![1, 6], vec![2, 7], vec![3, 8]]), 6);
        assert_eq!(func(vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![0, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![0, 9], vec![9, 10], vec![9, 12], vec![10, 11]]), 12);
    }
    test(count_good_nodes);
}
