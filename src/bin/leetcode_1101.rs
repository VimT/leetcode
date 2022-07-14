//! 彼此熟识的最早时间


struct UnionSet {
    f: Vec<usize>,
    size: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            size: vec![1; n],
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
        if xx == yy { return; }
        if self.size[xx] < self.size[yy] {
            std::mem::swap(&mut xx, &mut yy);
        }
        self.f[yy] = xx;
        self.size[xx] += self.size[yy];
    }
}


pub fn earliest_acq(mut logs: Vec<Vec<i32>>, n: i32) -> i32 {
    logs.sort_unstable();
    let mut us = UnionSet::new(n as usize);
    for log in logs {
        us.union(log[1] as usize, log[2] as usize);
        let root = us.find(log[1] as usize);
        if us.size[root] == n as usize {
            return log[0];
        }
    }
    -1
}

fn main() {
    fn test(func: fn(logs: Vec<Vec<i32>>, n: i32) -> i32) {
        assert_eq!(func(vec![vec![20190101, 0, 1], vec![20190104, 3, 4], vec![20190107, 2, 3], vec![20190211, 1, 5], vec![20190224, 2, 4], vec![20190301, 0, 3], vec![20190312, 1, 2], vec![20190322, 4, 5]], 6), 20190301);
        assert_eq!(func(vec![vec![0, 2, 0], vec![1, 0, 1], vec![3, 0, 3], vec![4, 1, 2], vec![7, 3, 1]], 4), 3);
    }
    test(earliest_acq);
}
