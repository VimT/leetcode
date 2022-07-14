//! 岛屿数量 II


struct UnionSet {
    f: Vec<usize>,
    size: Vec<usize>,
    count: usize, // set num
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: vec![n; n],
            size: vec![1; n],
            count: 0,
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

    fn isolate(&mut self, x: usize) {
        if self.f[x] != x {
            self.f[x] = x;
            self.size[x] = 1;
            self.count += 1;
        }
    }

    fn is_empty(&self, x: usize) -> bool {
        self.f[x] == self.f.len()
    }
}

/// 修改版并查集，最开始都指向len（都是水），添加岛屿==isolate隔离，最后看并查集的set数量
pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
    let mut us = UnionSet::new((m * n) as usize);
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    positions.into_iter().map(|pos| {
        let (x, y) = (pos[0], pos[1]);
        us.isolate((x * n + y) as usize);
        for (dx, dy) in DIR {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < m && ny >= 0 && ny < n {
                if !us.is_empty((nx * n + ny) as usize) {
                    us.union((nx * n + ny) as usize, (x * n + y) as usize);
                }
            }
        }
        us.count as i32
    }).collect()
}

fn main() {
    fn test(func: fn(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(3, 3, vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![2, 1]]), vec![1, 1, 2, 3]);
        assert_eq!(func(1, 1, vec![vec![0, 0]]), vec![1]);
    }
    test(num_islands2);
}
