//! 情侣牵手

pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
    let mut ans = 0;
    let len = row.len();
    for i in (0..len).step_by(2) {
        let b = row[i] ^ 1;
        if b == row[i + 1] { continue; }
        for j in 0..len {
            if row[j] == b {
                row.swap(j, i + 1);
                break;
            }
        }
        ans += 1;
    }
    ans
}


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

/// 至少交换的次数 = 所有情侣的对数 - 并查集里连通分量的个数
pub fn min_swaps_couples_us(row: Vec<i32>) -> i32 {
    let len = row.len();
    let n = len / 2;
    let mut us = UnionSet::new(n);
    for i in (0..len).step_by(2) {
        us.union((row[i] / 2) as usize, (row[i + 1] / 2) as usize);
    }
    (n - us.count) as i32
}

fn main() {
    assert_eq!(min_swaps_couples_us(vec![0, 2, 1, 3]), 1);
    assert_eq!(min_swaps_couples_us(vec![3, 2, 1, 0]), 0);
}
