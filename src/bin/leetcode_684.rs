//! 冗余连接

use std::mem;

struct SetUnion {
    f: Vec<usize>,
    rank: Vec<usize>,
}

impl SetUnion {
    fn new(n: usize) -> Self {
        SetUnion {
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
            mem::swap(&mut fx, &mut fy)
        }
        self.rank[fx] += self.rank[fy];
        self.f[fy] = fx;
    }
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let len = edges.len();
    let mut su = SetUnion::new(len);
    for i in 0..len {
        let e1 = edges[i][0] as usize - 1;
        let e2 = edges[i][1] as usize - 1;
        let x = su.find(e1);
        let y = su.find(e2);
        if x == y {
            return edges[i].clone();
        }
        su.union(e1, e2);
    }
    return vec![];
}


fn main() {
    assert_eq!(find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]), [2, 3]);
    assert_eq!(find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]]), [1, 4]);
}