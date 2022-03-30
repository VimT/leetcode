//! 移除最多的同行或同列石头


use std::collections::HashMap;

struct UnionSet {
    f: HashMap<usize, usize>,
    count: usize,
}

impl UnionSet {
    fn new() -> Self {
        UnionSet {
            f: HashMap::new(),
            count: 0,
        }
    }

    fn find(&mut self, n: usize) -> usize {
        return match self.f.get(&n) {
            None => {
                self.count += 1;
                self.f.insert(n, n);
                n
            }
            Some(&v) => {
                if v != n {
                    let vv = self.find(v);
                    self.f.insert(n, vv);
                    return vv;
                }
                return v;
            }
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let xx = self.find(x);
        let yy = self.find(y);
        if xx == yy {
            return;
        }
        self.f.insert(xx, yy);
        self.count -= 1;
    }
}

pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let mut us = UnionSet::new();
    let len = stones.len();
    for stone in stones {
        us.union(stone[0] as usize + 10001, stone[1] as usize);
    }
    return (len - us.count) as i32;
}


fn main() {
    assert_eq!(remove_stones(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]]), 5);
    assert_eq!(remove_stones(vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]]), 3);
    assert_eq!(remove_stones(vec![vec![0, 0]]), 0);
}
