//! 矩阵转换后的秩

use std::collections::{HashMap, VecDeque};
use leetcode::union_find::UnionFind;

pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut uf = UnionFind::new(n * m);
    for i in 0..m {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for j in 0..n {
            map.entry(matrix[i][j]).or_default().push(i * n + j);
        }
        for (_, v) in map.iter() {
            for k in &v[1..] {
                us.union(v[0], *k);
            }
        }
    }
    for j in 0..n {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..m {
            map.entry(matrix[i][j]).or_default().push(i * n + j);
        }
        for (_, v) in map.iter() {
            for k in &v[1..] {
                us.union(v[0], *k);
            }
        }
    }

    let mut outv = vec![vec![]; n * m];
    let mut ind = vec![0; n * m];
    for i in 0..m {
        let mut row: Vec<(i32, usize)> = matrix[i].iter().enumerate().map(|(x, y)| (*y, x)).collect();
        row.sort_unstable();
        for j in 0..n - 1 {
            if row[j].0 != row[j + 1].0 {
                let target = us.find(i * n + row[j + 1].1);
                outv[us.find(i * n + row[j].1)].push(target);
                ind[target] += 1;
            }
        }
    }
    for i in 0..n {
        let mut col = Vec::with_capacity(m);
        for j in 0..m {
            col.push((matrix[j][i], j));
        }
        col.sort_unstable();
        for j in 0..m - 1 {
            if col[j].0 != col[j + 1].0 {
                let target = us.find(col[j + 1].1 * n + i);
                outv[us.find(col[j].1 * n + i)].push(target);
                ind[target] += 1;
            }
        }
    }

    let mut q = VecDeque::new();
    for i in 0..n * m {
        if ind[i] == 0 && us.find(i) == i {
            q.push_back(i);
        }
    }
    let mut ans = vec![1; n * m];
    while !q.is_empty() {
        let xy = q.pop_front().unwrap();
        for &nxy in &outv[xy] {
            ind[nxy] -= 1;
            if ind[nxy] == 0 {
                ans[nxy] = ans[nxy].max(ans[xy] + 1);
                q.push_back(nxy);
            }
        }
    }
    let mut result = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            result[i][j] = ans[us.find(i * n + j)];
        }
    }
    result
}

fn main() {
    assert_eq!(matrix_rank_transform(vec![vec![20, -21, 14], vec![-19, 4, 19], vec![22, -47, 24], vec![-19, 4, 19]]), vec![vec![4, 2, 3], vec![1, 3, 4], vec![5, 1, 6], vec![1, 3, 4]]);
    assert_eq!(matrix_rank_transform(vec![vec![-24, -9, -14, -15, 44, 31, -46, 5, 20, -5, 34], vec![9, -40, -49, -50, 17, 40, 35, 30, -39, 36, -49], vec![-18, -43, -40, -5, -30, 9, -28, -41, -6, -47, 12], vec![11, 42, -23, 20, 35, 34, -39, -16, 27, 34, -15], vec![32, 27, -30, 29, -48, 15, -50, -47, -28, -21, 38], vec![45, 48, -1, -18, 9, -4, -13, 10, 9, 8, -41], vec![-42, -35, 20, -17, 10, 5, 36, 47, 6, 1, 8], vec![3, -50, -23, 16, 31, 2, -39, 36, -25, -30, 37], vec![-48, -41, 18, -31, -48, -1, -42, -3, -8, -29, -2], vec![17, 0, 31, -30, -43, -20, -37, -6, -43, 8, 19], vec![42, 25, 32, 27, -2, 45, 12, -9, 34, 17, 32]]
    ), vec![vec![4, 11, 10, 9, 25, 21, 2, 14, 20, 12, 24], vec![18, 5, 2, 1, 21, 25, 23, 22, 6, 24, 2], vec![8, 2, 5, 11, 6, 18, 7, 4, 10, 1, 20], vec![19, 24, 9, 20, 23, 22, 4, 10, 21, 22, 11], vec![23, 20, 6, 22, 2, 19, 1, 3, 7, 8, 26], vec![26, 27, 11, 7, 19, 9, 8, 20, 19, 14, 3], vec![3, 6, 21, 8, 20, 17, 24, 25, 18, 13, 19], vec![17, 1, 9, 18, 22, 16, 4, 23, 8, 5, 25], vec![2, 4, 16, 5, 2, 15, 3, 13, 9, 6, 14], vec![20, 13, 22, 6, 3, 7, 5, 12, 3, 14, 21], vec![25, 16, 23, 21, 12, 26, 13, 11, 24, 15, 23]]);
    assert_eq!(matrix_rank_transform(vec![vec![-37, -50, -3, 44], vec![-37, 46, 13, -32], vec![47, -42, -3, -40], vec![-17, -22, -39, 24]]), vec![vec![2, 1, 4, 6], vec![2, 6, 5, 4], vec![5, 2, 4, 3], vec![4, 3, 1, 5]]);
    assert_eq!(matrix_rank_transform(vec![vec![1, 2], vec![3, 4]]), vec![vec![1, 2], vec![2, 3]]);
    assert_eq!(matrix_rank_transform(vec![vec![7, 7], vec![7, 7]]), vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(matrix_rank_transform(vec![vec![7, 3, 6], vec![1, 4, 5], vec![9, 8, 2]]), vec![vec![5, 1, 4], vec![1, 2, 3], vec![6, 3, 1]]);
}

