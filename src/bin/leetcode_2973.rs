//! 树中每个节点放置的金币数目

pub fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
    let len = cost.len();
    let mut g = vec![vec![]; len];
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        g[a].push(b);
        g[b].push(a);
    }
    fn dfs(g: &Vec<Vec<usize>>, cost: &Vec<i32>, u: usize, fa: usize, result: &mut Vec<i64>) -> Vec<i64> {
        let mut nums = Vec::with_capacity(g[u].len() * 5 + 1);
        nums.push(cost[u] as i64);
        for &v in &g[u] {
            if v != fa { nums.extend(dfs(g, cost, v, u, result)); }
        }
        nums.sort_unstable();
        let len = nums.len();
        if len >= 3 {
            result[u] = (nums[len - 1] * nums[len - 2] * nums[len - 3])
                .max(nums[len - 1] * nums[0] * nums[1])
                .max(0);
        }
        if len >= 5 {
            nums = nums[..2].iter().chain(nums[len - 3..].iter()).copied().collect();
        }
        nums
    }
    let mut result = vec![1; len];
    dfs(&g, &cost, 0, len, &mut result);
    result
}

fn main() {
    fn test(func: fn(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64>) {
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]], vec![1, 2, 3, 4, 5, 6]), vec![120, 1, 1, 1, 1, 1]);
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![2, 8]], vec![1, 4, 2, 3, 5, 7, 8, -4, 2]), vec![280, 140, 32, 1, 1, 1, 1, 1, 1]);
        assert_eq!(func(vec![vec![0, 1], vec![0, 2]], vec![1, 2, -2]), vec![0, 1, 1]);
    }
    test(placed_coins);
}
