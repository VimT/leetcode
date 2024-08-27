//! 标记所有节点需要的时间

/// 换根DP，非常万能。。
pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }

    let mut f0 = vec![0; n]; // 以0为根的树上，标记所有节点需要的时间
    let mut f = vec![0; n]; // 以i为根的树上，标记所有节点需要的时间

    // 第一次dfs求 以0为根标记所有节点需要的时间
    fn dfs1(g: &Vec<Vec<usize>>, fa: usize, u: usize, f0: &mut Vec<i32>) {
        let mut max = 0;
        for &v in &g[u] {
            if v != fa {
                // 子树路径的最大值
                dfs1(g, u, v, f0);
                max = max.max(f0[v]);
            }
        }
        f0[u] = max + 2 - u as i32 % 2;
    }

    // 第二次dfs 求转移
    // 将整棵树的根节点从 r 换到 r 的一个相邻节点 r1
    // 交换之后，除了 r和r1 这两个节点，其他节点的 f0 是不变的。 这样只更新r和r1，即完成换根
    fn dfs2(g: &Vec<Vec<usize>>, f0: &mut Vec<i32>, f: &mut Vec<i32>, fa: usize, u: usize) {
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
                    f0[u] = m2 + 2 - u as i32 % 2;
                } else {
                    f0[u] = m1 + 2 - u as i32 % 2;
                }
                dfs2(g, f0, f, u, v);
            }
        }
    }

    dfs1(&g, n, 0, &mut f0);
    dfs2(&g, &mut f0, &mut f, n, 0);
    f
}


fn main() {
    fn test(func: fn(edges: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 0], vec![2, 1], vec![5, 3], vec![3, 1], vec![6, 1], vec![4, 3]]), vec![4, 3, 4, 3, 4, 4, 4]);
        assert_eq!(func(vec![vec![0, 1]]), vec![1, 2]);
        assert_eq!(func(vec![vec![0, 1], vec![0, 2]]), vec![2, 4, 3]);
        assert_eq!(func(vec![vec![2, 4], vec![0, 1], vec![2, 3], vec![0, 2]]), vec![4, 6, 3, 5, 5]);
    }
    test(time_taken);
}
