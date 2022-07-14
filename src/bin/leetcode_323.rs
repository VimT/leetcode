//! 无向图中连通分量的数目


struct UnionSet {
    f: Vec<usize>,
    size: Vec<usize>,
    count: usize, // set num
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut xx = self.find(x);
        let mut yy = self.find(y);
        if xx == yy {
            return;
        }
        if self.size[xx] < self.size[yy] {
            std::mem::swap(&mut xx, &mut yy);
        }
        self.f[yy] = xx;
        self.size[xx] += self.size[yy];
        self.count -= 1;
    }
}

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut us = UnionSet::new(n as usize);
    for edge in edges {
        us.union(edge[0] as usize, edge[1] as usize);
    }
    us.count as i32
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]), 2);
        assert_eq!(func(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]), 1);
    }
    test(count_components);
}
