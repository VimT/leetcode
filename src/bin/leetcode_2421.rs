//! 好路径的数目

use leetcode::union_set::UnionSet;

pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let len = vals.len();
    let mut g = vec![vec![]; len];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut us = UnionSet::new(len);
    let mut result = len;
    let mut vi: Vec<(i32, usize)> = vals.iter().enumerate().map(|x| (*x.1, x.0)).collect();
    vi.sort_unstable();
    for (vx, x) in vi {
        let fx = us.find(x);
        for &y in &g[x] {
            let y = us.find(y);
            if y == fx || vals[y] > vx { continue; }
            if vals[y] == vx { // 可以构成最好路径
                result += us.size[fx] * us.size[y];
                us.size[fx] += us.size[y]; // 连通块内节点值等于vx的节点个数
            }
            us.f[y] = fx;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(number_of_good_paths(vec![1, 3, 2, 1, 3], vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]), 6);
    assert_eq!(number_of_good_paths(vec![1, 1, 2, 2, 3], vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]]), 7);
    assert_eq!(number_of_good_paths(vec![1], vec![]), 1);
}
