//! 找到最小生成树里的关键边和伪关键边

use leetcode::union_find::UnionFind;

pub fn find_critical_and_pseudo_critical_edges(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for (i, edge) in edges.iter_mut().enumerate() {
        edge.push(i as i32);
    }
    edges.sort_unstable_by_key(|x| x[2]);
    let n = n as usize;
    let mut uf = UnionFind::new(n);
    let mut value = 0;

    fn unite(us: &mut UnionFind, edge: &Vec<i32>) -> bool {
        if us.find(edge[0] as usize) != us.find(edge[1] as usize) {
            us.union(edge[0] as usize, edge[1] as usize);
            return true;
        }
        false
    }

    for edge in &edges {
        if unite(&mut us, edge) {
            value += edge[2];
        }
    }
    let len = edges.len();
    let mut key = vec![];
    let mut ww = vec![];
    // 枚举每个边
    for i in 0..len {
        // 判断是否是关键边
        let mut uf = UnionFind::new(n);
        let mut v = 0;
        for (j, edge) in edges.iter().enumerate() {
            if i != j && unite(&mut us, edge) {
                v += edge[2];
            }
        }
        if us.count != 1 || v > value {
            key.push(edges[i][3]);
            continue;
        }

        // 判断是否是伪关键边
        us = UnionFind::new(n);
        unite(&mut us, &edges[i]);
        v = edges[i][2];
        for (j, edge) in edges.iter().enumerate() {
            if i != j && unite(&mut us, edge) {
                v += edge[2];
            }
        }
        if v == value {
            ww.push(edges[i][3]);
        }
    }

    vec![key, ww]
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(5, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 2], vec![0, 3, 2], vec![0, 4, 3], vec![3, 4, 3], vec![1, 4, 6]]), vec![vec![0, 1], vec![2, 3, 4, 5]]);
        assert_eq!(func(4, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]]), vec![vec![], vec![0, 1, 2, 3]]);
    }
    test(find_critical_and_pseudo_critical_edges);
}
