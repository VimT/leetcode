//! 在带权树网络中统计可连接服务器对数目

pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for edge in edges {
        let (a, b, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        g[a].push((b, w));
        g[b].push((a, w));
    }
    fn dfs(g: &Vec<Vec<(usize, i32)>>, u: usize, p: usize, cur: i32, speed: i32, cnt: &mut i32) {
        if cur % speed == 0 { *cnt += 1; }
        for &(v, w) in &g[u] {
            if v != p {
                dfs(g, v, u, cur + w, speed, cnt);
            }
        }
    }
    (0..n).map(|i| {
        let mut result = 0;
        let mut has = 0;
        for &(v, w) in &g[i] {
            let mut cnt = 0;
            dfs(&g, v, i, w, signal_speed, &mut cnt);
            result += cnt * has;
            has += cnt;
        }
        result
    }).collect()
}

fn main() {
    fn test(func: fn(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![0, 1, 1], vec![1, 2, 5], vec![2, 3, 13], vec![3, 4, 9], vec![4, 5, 2]], 1), vec![0, 4, 6, 6, 4, 0]);
        assert_eq!(func(vec![vec![0, 6, 3], vec![6, 5, 3], vec![0, 3, 1], vec![3, 2, 7], vec![3, 1, 6], vec![3, 4, 2]], 3), vec![2, 0, 0, 0, 0, 0, 2]);
    }
    test(count_pairs_of_connectable_servers);
}
