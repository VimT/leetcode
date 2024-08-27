//! 新增道路查询后的最短距离 II

use std::collections::BTreeSet;

/// 把中间的捷径城市都删了
pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut c: BTreeSet<i32> = (0..n).collect();
    queries.into_iter().map(|q| {
        let (u, v) = (q[0], q[1]);
        for val in c.range(u + 1..v).copied().collect::<Vec<i32>>() {
            c.remove(&val);
        }
        (c.len() - 1) as i32
    }).collect()
}

/// 区间并查集
pub fn shortest_distance_after_queries2(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    struct SimpleUnionFind {
        f: Vec<usize>,
        count: usize,
    }
    impl SimpleUnionFind {
        fn new(n: usize) -> Self {
            Self { f: (0..n).collect(), count: n }
        }
        fn find(&mut self, x: usize) -> usize {
            if self.f[x] != x {
                self.f[x] = self.find(self.f[x]);
            }
            self.f[x]
        }
        fn union(&mut self, x: usize, y: usize) {
            let (px, py) = (self.find(x), self.find(y));
            if px != py {
                self.f[px] = py;
                self.count -= 1;
            }
        }
    }
    let n = n as usize;
    let mut uf = SimpleUnionFind::new(n - 1);
    queries.into_iter().map(|q| {
        let (l, r) = (q[0] as usize, q[1] as usize);
        let fr = uf.find(r - 1);
        let mut i = uf.find(l);
        while i < r - 1 {
            uf.union(i, fr);
            i = uf.find(i + 1);
        }
        uf.count as i32
    }).collect()
}


fn main() {
    fn test(func: fn(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]), vec![3, 2, 1]);
        assert_eq!(func(4, vec![vec![0, 3], vec![0, 2]]), vec![1, 1]);
    }
    test(shortest_distance_after_queries);
    test(shortest_distance_after_queries2);
}
