//! 水资源分配优化

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

/// 最小生成树
pub fn min_cost_to_supply_water(n: i32, mut wells: Vec<i32>, mut pipes: Vec<Vec<i32>>) -> i32 {
    let mut us = UnionSet::new(n as usize);
    pipes.sort_unstable_by_key(|x| x[2]);
    let mut result = wells.iter().sum();
    for pipe in pipes {
        let a = pipe[0] as usize - 1;
        let b = pipe[1] as usize - 1;
        let aroot = us.find(a);
        let broot = us.find(b);
        if aroot != broot {
            // 决定铺还是不铺
            let aprice = wells[aroot];
            let bprice = wells[broot];
            if (aprice.min(bprice) + pipe[2]) < (aprice + bprice) {
                us.union(a, b);
                wells[us.find(a)] = aprice.min(bprice);
                result -= aprice.max(bprice);
                result += pipe[2];
            }
        }
    }

    result
}

fn main() {
    fn test(func: fn(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, vec![1, 2, 2], vec![vec![1, 2, 1], vec![2, 3, 1]]), 3);
        assert_eq!(func(2, vec![1, 1], vec![vec![1, 2, 1], vec![1, 2, 2]]), 2);
    }
    test(min_cost_to_supply_water);
}
