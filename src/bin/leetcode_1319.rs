//! number-of-operations-to-make-network-connected


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

pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let len = connections.len();
    let n = n as usize;
    if len + 1 < n {
        return -1;
    }
    let mut us = UnionSet::new(n);
    for connection in connections {
        us.union(connection[0] as usize, connection[1] as usize);
    }
    (us.count - 1) as i32
}

fn main() {
    assert_eq!(make_connected(5, vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]]), 0);
    assert_eq!(make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 1);
    assert_eq!(make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]), 2);
    assert_eq!(make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]), -1);
}
