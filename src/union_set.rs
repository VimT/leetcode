use std::collections::HashMap;
use std::hash::Hash;

pub struct UnionSet {
    pub f: Vec<usize>,
    pub size: Vec<usize>,
    pub count: usize, // set num
}

impl UnionSet {
    pub fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let mut parent = self.find(x);
        let mut son = self.find(y);
        if parent == son {
            return;
        }
        if self.size[parent] < self.size[son] {
            std::mem::swap(&mut parent, &mut son);
        }
        self.f[son] = parent;
        self.size[parent] += self.size[son];
        self.count -= 1;
    }

    pub fn union_force(&mut self, parent: usize, son: usize) {
        let parent = self.find(parent);
        let son = self.find(son);
        if parent == son {
            return;
        }
        self.f[son] = parent;
        self.size[parent] += son;
        self.count -= 1;
    }

    pub fn isolate(&mut self, x: usize) {
        if self.f[x] != x {
            self.f[x] = x;
            self.size[x] = 1;
            self.count += 1;
        }
    }

    pub fn is_empty(&self, x: usize) -> bool {
        self.f[x] == self.f.len()
    }

    // 当前节点所属集合的size
    pub fn size(&mut self, x: usize) -> usize {
        let xx = self.find(x);
        self.size[xx]
    }
}


pub struct UnionSetHashMap<K> {
    f: HashMap<K, K>,
    pub count: usize,
}

impl<K: Copy + Eq + Hash> UnionSetHashMap<K> {
    pub fn new() -> Self {
        Self {
            f: HashMap::new(),
            count: 0,
        }
    }

    pub fn find(&mut self, n: K) -> K {
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

    pub fn union(&mut self, parent: K, son: K) {
        let parent = self.find(parent);
        let son = self.find(son);
        if parent == son {
            return;
        }
        self.f.insert(son, parent);
        self.count -= 1;
    }
}
