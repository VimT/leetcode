//! 合并两棵树后的最小直径

/// 换根dp 求以 u 为根结点时，树高的最大值 和 次大值
pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    fn find_max(edges: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push(edge[1] as usize);
            g[edge[1] as usize].push(edge[0] as usize);
        }

        let mut f0 = vec![0; n]; // 以0为根的树上，各节点的路径最大值
        let mut f = vec![(0, 0); n]; // 以i为根的路径最大值 和 次大值

        // 第一次dfs求 以0为根的路径最大值
        fn dfs1(g: &Vec<Vec<usize>>, fa: usize, u: usize, f0: &mut Vec<i32>) {
            let mut max = 0;
            for &v in &g[u] {
                if v != fa {
                    // 子树路径的最大值
                    dfs1(g, u, v, f0);
                    max = max.max(f0[v]);
                }
            }
            f0[u] = max + 1;
        }

        // 第二次dfs 求转移
        // 将整棵树的根节点从 r 换到 r 的一个相邻节点 r1
        // 交换之后，除了 r和r1 这两个节点，其他节点的 f0 是不变的。 这样只更新r和r1，即完成换根
        fn dfs2(g: &Vec<Vec<usize>>, f0: &mut Vec<i32>, f: &mut Vec<(i32, i32)>, fa: usize, u: usize) {
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
            f[u] = (m1, m2);

            // 换根到v
            for &v in &g[u] {
                if v != fa {
                    // v为根节点后，原v是最大路径上的点。要换成次大路径
                    if f0[v] == m1 {
                        f0[u] = m2 + 1;
                    } else {
                        f0[u] = m1 + 1;
                    }
                    dfs2(g, f0, f, u, v);
                }
            }
        }

        dfs1(&g, n, 0, &mut f0);
        dfs2(&g, &mut f0, &mut f, n, 0);
        f
    }

    let x1 = find_max(&edges1);
    let x2 = find_max(&edges2);
    let x1_min = x1.iter().min_by_key(|x| x.0).unwrap();
    let x2_min = x2.iter().min_by_key(|x| x.0).unwrap();
    (x1_min.0 + x1_min.1).max(x2_min.0 + x2_min.1).max(x1_min.0 + x2_min.0 + 1)
}

/// 直接计算两个树的直径
pub fn minimum_diameter_after_merge2(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    fn diameter(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push(edge[1] as usize);
            g[edge[1] as usize].push(edge[0] as usize);
        }
        let mut result = 0;
        fn dfs(g: &Vec<Vec<usize>>, u: usize, fa: usize, result: &mut i32) -> i32 {
            let mut max_len = 0;
            for &v in &g[u] {
                if v != fa {
                    let sub_len = dfs(g, v, u, result) + 1;
                    *result = (*result).max(max_len + sub_len);
                    max_len = max_len.max(sub_len);
                }
            }
            max_len
        }
        dfs(&g, 0, n, &mut result);
        result
    }
    let d1 = diameter(edges1);
    let d2 = diameter(edges2);
    d1.max(d2).max((d1 + 1) / 2 + (d2 + 1) / 2 + 1)
}

fn main() {
    fn test(func: fn(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![0, 3]], vec![vec![0, 1]]), 3);
        assert_eq!(func(vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![2, 7]], vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![2, 7]]), 5);
    }
    test(minimum_diameter_after_merge);
    test(minimum_diameter_after_merge2);
}
