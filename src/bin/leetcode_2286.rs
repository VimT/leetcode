//! 以组为单位订音乐会的门票

struct BookMyShow {
    m: u64,
    n: u64,
    // 总已使用
    sum: Vec<u64>,
    min: Vec<u64>, // 最小已使用
}


impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        Self {
            m: m as u64,
            n: n as u64,
            sum: vec![0; 4 * n as usize],
            min: vec![0; 4 * n as usize],
        }
    }

    fn _gather(&mut self, s: usize, t: usize, p: usize, k: u64, max_row: usize) -> Option<(i32, i32)> {
        if s > max_row || self.m - self.min[p] < k {
            return None;
        }
        if s == t {
            self.min[p] += k;
            self.sum[p] += k;
            return Some((s as i32, (self.min[p] - k) as i32));
        }
        let mid = (s + t) / 2;
        let left = self._gather(s, mid, p * 2, k, max_row);
        if left.is_some() {
            self.min[p] = self.min[p * 2].min(self.min[p * 2 + 1]);
            self.sum[p] = self.sum[p * 2] + self.sum[p * 2 + 1];
            return left;
        }
        let right = self._gather(mid + 1, t, p * 2 + 1, k, max_row);
        if right.is_some() {
            self.min[p] = self.min[p * 2].min(self.min[p * 2 + 1]);
            self.sum[p] = self.sum[p * 2] + self.sum[p * 2 + 1];
        }
        right
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let result = self._gather(0, (self.n - 1) as usize, 1, k as u64, max_row as usize);
        match result {
            Some(v) => vec![v.0, v.1],
            None => vec![]
        }
    }

    fn _sum(&mut self, s: usize, t: usize, p: usize, max_row: usize) -> u64 {
        if self.min[p] == self.m {
            // 已经满了
            return (t.min(max_row) - s + 1) as u64 * self.m;
        }
        if t <= max_row {
            return self.sum[p];
        }
        let mid = (s + t) / 2;
        let mut result = self._sum(s, mid, p * 2, max_row);
        if max_row > mid {
            result += self._sum(mid + 1, t, p * 2 + 1, max_row);
        }
        result
    }

    fn _cost(&mut self, s: usize, t: usize, p: usize, k: u64, max_row: usize) -> u64 {
        if s > max_row { return 0; }
        if t <= max_row {
            let avail = (t - s + 1) as u64 * self.m - self.sum[p];
            if k >= avail {
                // 全部使用
                self.sum[p] = (t - s + 1) as u64 * self.m;
                self.min[p] = self.m;
                return avail;
            }
            if s == t {
                let avail = self.m - self.sum[p];
                let cost = avail.min(k);
                self.sum[p] += cost;
                self.min[p] += cost;
                return cost;
            }
        }
        let mid = (s + t) / 2;
        let mut result = 0;
        let left = self._cost(s, mid, p * 2, k, max_row);
        result += left;
        if k - left > 0 {
            result += self._cost(mid + 1, t, p * 2 + 1, k - left, max_row);
        }
        self.min[p] = self.min[p * 2].min(self.min[p * 2 + 1]);
        self.sum[p] = self.sum[p * 2] + self.sum[p * 2 + 1];
        result
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let used = self._sum(0, self.n as usize - 1, 1, max_row as usize);
        if (max_row as u64 + 1) * self.m as u64 - used < k as u64 {
            return false;
        }
        self._cost(0, self.n as usize - 1, 1, k as u64, max_row as usize);
        true
    }

    fn _print(&self, s: usize, t: usize, p: usize) {
        if s == t {
            println!("{}: {}", s, self.min[p]);
            return;
        }
        if self.min[p] == self.m {
            for i in s..=t {
                println!("{}: {}", i, self.m);
            }
            return;
        }
        let mid = (s + t) / 2;
        self._print(s, mid, p * 2);
        self._print(mid + 1, t, p * 2 + 1);
    }

    fn print(&self) {
        self._print(0, self.n as usize - 1, 1);
        println!("---");
    }
}

/// 60ms第一的
/// 使用 2n 长度作为线段树，n/2+idx 就是叶子节点。n*2+1是左节点，n*2+2是右节点，根是0，父节点是(idx-1)/2
/// 遍历的时候不需要递归，直接循环就可以。更新的时候，可以直接找到叶子节点，从叶子节点向上更新
mod other {
    struct BookMyShow {
        max_tree: MaxTree,
        sum_tree: SumTree,
        m: i64,
        first_not_full: usize,
    }

    struct MaxTree {
        tree: Vec<i64>,
    }

    impl MaxTree {
        pub fn new(n: usize) -> Self {
            let mut tree = vec![0; 2 * n - 1];
            Self { tree }
        }

        pub fn query_first_ge(&self, min: i64) -> Option<(usize, i64)> {
            let n = self.tree.len();
            let mut node = 0;
            if self.tree[node] < min {
                return None;
            }
            while 2 * node + 1 < n {
                if self.tree[2 * node + 1] >= min {
                    node = 2 * node + 1;
                } else {
                    node = 2 * node + 2;
                }
            }
            Some((node - n / 2, self.tree[node]))
        }

        pub fn update(&mut self, idx: usize, val: i64) {
            let n = self.tree.len();
            let mut idx = idx + n / 2;
            self.tree[idx] = val;
            while idx > 0 {
                let p = (idx - 1) / 2;
                let c1 = 2 * p + 1;
                let c2 = 2 * p + 2;
                let max = if c2 < n {
                    self.tree[c1].max(self.tree[c2])
                } else {
                    self.tree[c1]
                };
                if self.tree[p] == max {
                    break;
                }
                self.tree[p] = max;
                idx = p;
            }
        }
    }

    struct SumTree {
        tree: Vec<i64>,
    }

    impl SumTree {
        pub fn new(n: usize) -> Self {
            let mut tree = vec![0; 2 * n - 1];
            Self { tree }
        }

        pub fn query_first_ge(&self, mut min: i64) -> Option<(usize, i64)> {
            let n = self.tree.len();
            let mut node = 0;
            if self.tree[node] < min {
                return None;
            }
            while 2 * node + 1 < n {
                if self.tree[2 * node + 1] >= min {
                    node = 2 * node + 1;
                } else {
                    min -= self.tree[2 * node + 1];
                    node = 2 * node + 2;
                }
            }
            Some((node - n / 2, self.tree[node] - min))
        }

        pub fn update(&mut self, idx: usize, val: i64) {
            let n = self.tree.len();
            let mut idx = idx + n / 2;
            let delta = val - self.tree[idx];
            self.tree[idx] = val;
            while idx > 0 {
                let p = (idx - 1) / 2;
                self.tree[p] += delta;
                idx = p;
            }
        }
    }


    impl BookMyShow {
        fn new(n: i32, m: i32) -> Self {
            let (n, m) = (n as usize, m as i64);
            let mut cap = 1;
            while cap < n {
                cap *= 2;
            }
            let mut max_tree = MaxTree::new(cap);
            for i in 0..n {
                max_tree.update(i, m);
            }
            let mut sum_tree = SumTree::new(cap);
            for i in 0..n {
                sum_tree.update(i, m);
            }
            //println!("{:?} {:?}", max_tree.tree, sum_tree.tree);
            Self {
                max_tree,
                sum_tree,
                m,
                first_not_full: 0,
            }
        }

        fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
            let (k, max_row) = (k as i64, max_row as usize);
            if let Some((idx, space)) = self.max_tree.query_first_ge(k) {
                if idx <= max_row {
                    self.max_tree.update(idx, space - k);
                    self.sum_tree.update(idx, space - k);
                    return vec![idx as i32, (self.m - space) as i32];
                }
            }
            vec![]
        }

        fn scatter(&mut self, k: i32, max_row: i32) -> bool {
            let (k, max_row) = (k as i64, max_row as usize);
            if let Some((idx, remain_space)) = self.sum_tree.query_first_ge(k) {
                if idx <= max_row {
                    for i in self.first_not_full..idx {
                        self.sum_tree.update(i, 0);
                        self.max_tree.update(i, 0);
                    }
                    self.sum_tree.update(idx, remain_space);
                    self.max_tree.update(idx, remain_space);
                    if remain_space > 0 {
                        self.first_not_full = idx;
                    } else {
                        self.first_not_full = idx + 1;
                    }
                    return true;
                }
            }
            false
        }
    }
}

fn main() {
    let mut bms = BookMyShow::new(2, 5); // 总共有 2 排，每排 5 个座位。
    assert_eq!(bms.gather(4, 0), [0, 0]); // 返回 [0, 0] 这一组安排第 0 排 [0, 3] 的座位。
    assert_eq!(bms.gather(2, 0).is_empty(), true); // 返回 [] 第 0 排只剩下 1 个座位。 所以无法安排 2 个连续座位。
    assert_eq!(bms.scatter(5, 1), true); // 返回 True 这一组安排第 0 排第 4 个座位和第 1 排 [0, 3] 的座位。
    assert_eq!(bms.scatter(5, 1), false); // 返回 False 总共只剩下 2 个座位。

    bms = BookMyShow::new(5, 3);
    assert_eq!(bms.scatter(3, 2), true);
    assert_eq!(bms.gather(10, 2).is_empty(), true);
    assert_eq!(bms.gather(1, 1), vec![1, 0]);
}

