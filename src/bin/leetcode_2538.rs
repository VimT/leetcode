//! 最大价值和与最小价值和的差值

use std::collections::HashMap;

/// O(3n) （如果不是树，而是菊花图，复杂度为 O(n^2) ）
pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
    fn dfs(price: &Vec<i32>, g: &Vec<Vec<usize>>, parent: usize, u: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
        if let Some(&v) = cache.get(&(parent, u)) {
            return v;
        }
        let mut result = 0;
        for &v in &g[u] {
            if v != parent {
                result = result.max(dfs(price, g, u, v, cache));
            }
        }
        result += price[u] as i64;
        cache.insert((parent, u), result);
        result
    }
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut result = 0;
    let mut cache = HashMap::new();
    for i in 0..n {
        if g[i].len() == 1 {
            result = result.max(dfs(&price, &g, i, g[i][0], &mut cache));
        }
    }
    result
}

/// 树上DP，O(n)。 -- 灵茶山艾府
pub fn max_output2(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut result = 0;
    /// 返回 （带上端点的最大路径和，不带上端点的最大路径和）
    fn dfs(price: &Vec<i32>, g: &Vec<Vec<usize>>, fa: usize, u: usize, result: &mut i64) -> (i64, i64) {
        let p = price[u] as i64;
        let mut max_s1 = p;
        let mut max_s2 = 0;
        for &v in &g[u] {
            if v != fa {
                let (s1, s2) = dfs(price, g, u, v, result);
                *result = (*result).max(s1 + max_s2).max(s2 + max_s1);  // 如果放下面可能就会路径重复
                max_s1 = max_s1.max(s1 + p);
                max_s2 = max_s2.max(s2 + p);
            }
        }
        (max_s1, max_s2)
    }
    dfs(&price, &g, n, 0, &mut result);
    result
}


/// 换根DP：从一个已知节点推导相连节点的大小
pub fn max_output3(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }

    let mut f0 = vec![0; n]; // 以0为根的树上，各节点的路径最大值
    let mut f = vec![0; n]; // 以i为根的路径最大值（不包括i）

    // 第一次dfs求 以0为根的路径最大值
    fn dfs1(g: &Vec<Vec<usize>>, price: &Vec<i32>, fa: usize, u: usize, f0: &mut Vec<i64>) {
        let mut max = 0;
        for &v in &g[u] {
            if v != fa {
                // 子树路径的最大值
                dfs1(g, price, u, v, f0);
                max = max.max(f0[v]);
            }
        }
        f0[u] = max + price[u] as i64;
    }

    // 第二次dfs 求转移
    // 将整棵树的根节点从 r 换到 r 的一个相邻节点 r1
    // 交换之后，除了 r和r1 这两个节点，其他节点的 f0 是不变的。 这样只更新r和r1，即完成换根
    fn dfs2(g: &Vec<Vec<usize>>, price: &Vec<i32>, f0: &mut Vec<i64>, f: &mut Vec<i64>, fa: usize, u: usize) {
        let mut m1 = 0; // 子树路径的最大值
        let mut m2 = 0; // 子树路径的次大值
        for &v in &g[u] {
            // u为根时的 子树路径的最大值和次大值
            if f0[v] > m1 {
                m2 = m1;
                m1 = f0[v];
            } else if f0[v] > m2 {
                m2 = f0[v];
            }
        }

        // 换根后的 当前路径最大值 可能的变化
        f[u] = m1;

        // 换根到v
        for &v in &g[u] {
            if v != fa {
                // v为根节点后，原v是最大路径上的点。要换成次大路径
                if f0[v] == m1 {
                    f0[u] = m2 + price[u] as i64;
                } else {
                    f0[u] = m1 + price[u] as i64;
                }
                dfs2(g, price, f0, f, u, v);
            }
        }
    }

    dfs1(&g, &price, n, 0, &mut f0);
    dfs2(&g, &price, &mut f0, &mut f, n, 0);
    f.into_iter().max().unwrap()
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64) {
        assert_eq!(func(6, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]], vec![9, 8, 7, 6, 10, 5]), 24);
        assert_eq!(func(3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1]), 2);
    }
    test(max_output);
    test(max_output2);
    test(max_output3);
}
