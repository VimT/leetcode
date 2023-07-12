//! 二维网格图中探测环

use std::collections::VecDeque;

pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut seen = vec![vec![0; n]; m];
    let mut q = VecDeque::new();
    let mut cnt = 1;
    for i in 0..m {
        for j in 0..n {
            if seen[i][j] == 0 {
                let ch = grid[i][j];
                q.push_back((i, j, m, n));
                seen[i][j] = cnt;
                while !q.is_empty() {
                    let (x, y, fx, fy) = q.pop_front().unwrap();
                    for (dx, dy) in DIR {
                        let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                        if nx < m && ny < n && (nx, ny) != (fx, fy) && grid[nx][ny] == ch {
                            if seen[nx][ny] == cnt { return true; } else if seen[nx][ny] == 0 {
                                seen[nx][ny] = cnt;
                                q.push_back((nx, ny, x, y));
                            }
                        }
                    }
                }
                cnt += 1;
            }
        }
    }
    false
}


/// 并查集，上下关系 和 左右关系 看成有一条边，我们要找是否有环
pub fn contains_cycle2(grid: Vec<Vec<char>>) -> bool {
    pub struct UnionFind {
        pub f: Vec<usize>,
        pub size: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            UnionFind {
                f: (0..n).collect(),
                size: vec![1; n],
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

        pub fn union(&mut self, x: usize, y: usize) -> bool {
            let mut parent = self.find(x);
            let mut son = self.find(y);
            if parent == son {
                return false;
            }
            if self.size[parent] < self.size[son] {
                std::mem::swap(&mut parent, &mut son);
            }
            self.f[son] = parent;
            self.size[parent] += self.size[son];
            return true;
        }
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut us = UnionFind::new(m * n);
    for i in 0..m {
        for j in 0..n {
            if i > 0 && grid[i][j] == grid[i - 1][j] {
                if !us.union(i * n + j, (i - 1) * n + j) {
                    return true;
                }
            }
            if j > 0 && grid[i][j] == grid[i][j - 1] {
                if !us.union(i * n + j, i * n + j - 1) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<char>>) -> bool) {
        assert_eq!(func(vec![vec!['a', 'b', 'b'], vec!['b', 'z', 'b'], vec!['b', 'b', 'a']]), false);
        assert_eq!(func(vec![vec!['a', 'a', 'a', 'a'], vec!['a', 'b', 'b', 'a'], vec!['a', 'b', 'b', 'a'], vec!['a', 'a', 'a', 'a']]), true);
        assert_eq!(func(vec![vec!['c', 'c', 'c', 'a'], vec!['c', 'd', 'c', 'c'], vec!['c', 'c', 'e', 'c'], vec!['f', 'c', 'c', 'c']]), true);
    }
    test(contains_cycle);
    test(contains_cycle2);
}
