//! 矩阵中严格递增的单元格数

use std::collections::{BTreeMap, VecDeque};

/// dp
pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();

    let mut g: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
    for i in 0..m {
        for j in 0..n {
            g.entry(mat[i][j]).or_default().push((i, j));
        }
    }
    let mut result = 0;
    let mut row_max = vec![0; m];
    let mut col_max = vec![0; n];
    for (_, pos) in g {
        let mx: Vec<i32> = pos.iter().map(|&(i, j)| row_max[i].max(col_max[j]) + 1).collect();
        result = result.max(mx.iter().max().copied().unwrap());
        for ((i, j), f) in pos.into_iter().zip(mx) {
            row_max[i] = row_max[i].max(f);
            col_max[j] = col_max[j].max(f);
        }
    }
    result
}


/// 建图法，大小临边建图+虚拟节点
pub fn max_increasing_cells2(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut g = vec![vec![]; m * n];
    for i in 0..m {
        let mut items: Vec<(i32, usize)> = mat[i].iter().copied().zip(0..).collect();
        items.sort_unstable();

        // 前一个值区间为 [p, j) ，当前值区间为 [j, q)
        let mut p = usize::MAX;
        let mut j = 0;
        while j < n {
            let mut q = j + 1;
            while q < n && items[q].0 == items[j].0 {
                q += 1;
            }
            if p != usize::MAX {
                let t = g.len(); // 虚拟节点
                g.push(vec![]);
                for k in p..j {
                    g[n * i + items[k].1].push(t);
                }
                for k in j..q {
                    g[t].push(n * i + items[k].1);
                }
            }
            p = j;
            j = q;
        }
    }

    for j in 0..n {
        let mut items: Vec<(i32, usize)> = vec![(0, 0); m];
        for i in 0..m {
            items[i] = (mat[i][j], i)
        }
        items.sort_unstable();

        // 前一个值区间为 [p, i) ，当前值区间为 [i, q)
        let mut p = usize::MAX;
        let mut i = 0;
        while i < m {
            let mut q = i + 1;
            while q < m && items[q].0 == items[i].0 {
                q += 1;
            }
            if p != usize::MAX {
                let t = g.len(); // 虚拟节点
                g.push(vec![]);
                for k in p..i {
                    g[n * items[k].1 + j].push(t);
                }
                for k in i..q {
                    g[t].push(n * items[k].1 + j);
                }
            }
            p = i;
            i = q;
        }
    }

    // 拓扑排序
    let len = g.len();
    let mut degree = vec![0; len];
    for edge in &g {
        for &v in edge {
            degree[v] += 1;
        }
    }
    let mut q = VecDeque::new();
    let mut result = 0;
    for i in 0..len {
        if degree[i] == 0 {
            q.push_back((i, 1));
        }
    }
    while !q.is_empty() {
        let (u, val) = q.pop_front().unwrap();
        result = result.max(val);
        for &v in &g[u] {
            degree[v] -= 1;
            if degree[v] == 0 {
                q.push_back((v, val + (v < m * n) as i32));
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![-4, 8, -3, 2, -4, -8, 7, 5, -2], vec![-5, 5, -7, -2, 6, -6, -8, -4, -4]]), 11);
        assert_eq!(func(vec![vec![3, 1, 6], vec![-9, 5, 7]]), 4);
        assert_eq!(func(vec![vec![3, 1], vec![3, 4]]), 2);
        assert_eq!(func(vec![vec![1, 1], vec![1, 1]]), 1);
    }
    test(max_increasing_cells);
    test(max_increasing_cells2);
}
