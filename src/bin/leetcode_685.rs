//! 冗余连接 II

use std::mem::swap;

struct UnionSet {
    f: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x { x } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut fx = self.find(x);
        let mut fy = self.find(y);
        if fx == fy { return; }
        if self.rank[fx] < self.rank[fy] {
            swap(&mut fx, &mut fy)
        }
        self.rank[fx] += self.rank[fy];
        self.f[fy] = fx;
    }
}

/// 两种情况：
/// 1。 附加边指向根节点：肯定有环
/// 2。 附加边指向其他节点：可能有环，必有节点有两个父
/// 所以 并查集找环，parent数组找重复父节点
pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let len = edges.len();
    let mut us = UnionSet::new(len + 1);
    let mut parent = vec![0; len + 1];
    for i in 1..=len {
        parent[i] = i;
    }
    let mut conflict = None;
    let mut cycle = None;
    for i in 0..len {
        let u = edges[i][0] as usize;
        let v = edges[i][1] as usize;
        if parent[v] != v {
            conflict = Some(i);
        } else {
            parent[v] = u;
            if us.find(u) == us.find(v) {
                cycle = Some(i);
            } else {
                us.union(u, v);
            }
        }
    }
    if let None = conflict {
        return edges[cycle.unwrap()].clone();
    }
    let conflict = conflict.unwrap();
    let conflict_edge = &edges[conflict];
    return if let Some(_) = cycle {
        // 如果有导致环路的边，则附加的边不可能是 [u,v]
        // （因为 [u,v] 已经被记为导致冲突的边，不可能被记为导致环路出现的边），因此附加的边是[parent[v],v]。
        vec![parent[conflict_edge[1] as usize] as i32, conflict_edge[1] as i32]
    } else {
        // 如果没有导致环路的边，则附加的边是后被访问到的指向v 的边，因此附加的边是 [u,v]
        conflict_edge.clone()
    };
}

fn main() {
    assert_eq!(find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]), vec![2, 3]);
    assert_eq!(find_redundant_directed_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]]), vec![4, 1]);
}
